use bevy::prelude::*;
use iyes_loopless::prelude::*;

use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_renet::{
  RenetClientPlugin,
  renet::{
    RenetClient,
    ConnectToken
  }
};
use renet_visualizer::{RenetClientVisualizer, RenetVisualizerStyle};
use bevy_egui::EguiContext;
use futures_lite::future;
use serde_json::Value as JsonValue;
use { bincode, reqwest, base64 };
use std::{
  time::{SystemTime},
  net::{SocketAddr, UdpSocket},
  io::Cursor
};
use shared::{
  types::{
    net::Lobby, 
    player::{Username, PlayerInitData},
    chunk::{Chunk, ChunkPosition, ChunkDataComponent},
  },
  messages::{
    ClientToServerMessages, 
    ServerToClientMessages,
  },
  consts::{
    CHANNEL_RELIABLE, CHANNEL_UNRELIABLE,
    renet_connection_config
  },
};
use crate::{
  GameState,
  chat::ChatMessages,
  player::{ChunkLocation, NetPlayer, Player},
  player::MainPlayer,
};

#[derive(Clone, Copy, Debug)]
pub struct RequestChunk(i64, i64);
impl From<ChunkPosition> for RequestChunk {
  fn from(from: ChunkPosition) -> Self { Self(from.0, from.1) }
}
impl From<ChunkLocation> for RequestChunk {
  fn from(from: ChunkLocation) -> Self { Self(from.0, from.1) }
}

#[derive(Clone, Debug)]
pub struct RequestNetChatSend(pub String);

#[derive(Clone, Debug)]
pub struct AddNetPlayer{
  pub client_id: u64,
  pub init_data: PlayerInitData
}

#[derive(Component)]
pub struct DecompressTask(pub Task<ChunkDataComponent>);

fn run_if_client_conected(client: Option<Res<RenetClient>>) -> bool {
  if let Some(client) = client {
    return client.is_connected();
  }
  false
}

#[derive(Clone, Debug)]
pub struct ConnectionConfig {
  pub addr: SocketAddr,
  pub username: String,
  pub password: Option<String>,
}

fn create_renet_client(
  mut commands: Commands,
  config: Res<ConnectionConfig>,
) {
  let api_url = format!("http://{}:{}",config.addr.ip(), config.addr.port());
  let addr_no_port = SocketAddr::new(config.addr.ip(), 0);

  //Get connection data
  let conn_data: JsonValue = {
    let client = reqwest::blocking::Client::new();
    let res = client.get(format!("{}/connect", api_url))
      .query(&[
        ("username", config.username.clone().as_str()),
        ("password", config.password.as_ref().map_or("", |s| s.as_ref())) //TODO don't pass empty password
      ])
      .send()
      .expect("Failed to get the connection token");
    let res_bytes = &res.bytes().unwrap()[..];
    serde_json::from_slice(res_bytes).unwrap()
  };

  //Parse it
  let (connect_token, client_id) = (
    {
      if !conn_data["success"].as_bool().unwrap_or_default() {
        error!("Connection failed; {}", conn_data["reason"].as_str().unwrap_or("<no reason>"));
        return;
      }
      let token_base64 = conn_data["token"].as_str().expect("No token in response");
      let token_bytes = base64::decode(token_base64).expect("Invalid token Base64");
      ConnectToken::read(&mut Cursor::new(&token_bytes)).unwrap()
    },
    conn_data["client_id"].as_u64().expect("No Client ID in response"),
  );
  
  //Bind socket
  let socket = UdpSocket::bind(addr_no_port).unwrap();

  //Create config things
  let connection_config = renet_connection_config();
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

  let client = RenetClient::new(current_time, socket, client_id, connect_token, connection_config).unwrap();
  commands.insert_resource(client);

  //Create a lobby resource
  commands.insert_resource(Lobby::default());

  info!("Client started");
}

fn update_state_to_ingame(
  mut commands: Commands,
  client: Res<RenetClient>
) {
  if client.is_connected() {
    commands.insert_resource(NextState(GameState::InGame));
  } else if client.disconnected().is_some() {
    commands.insert_resource(NextState(GameState::MainMenu));
  }
}

//TODO!!! Client: Separate into multiple systems
fn handle_incoming_stuff(
  mut commands: Commands,
  mut client: ResMut<RenetClient>,
  pool: Res<AsyncComputeTaskPool>,
  mut chat: ResMut<ChatMessages>,
  mut main_plr: Query<(Entity, &mut Transform), (With<MainPlayer>, Without<NetPlayer>)>,
  mut add_net_plr: EventWriter<AddNetPlayer>,
  mut net_plr_trans: Query<&mut Transform, (Without<MainPlayer>, With<NetPlayer>)>,
  lobby: ResMut<Lobby>,
) {
  if !client.is_connected() { return; }

  let mut main_plr = main_plr.single_mut();
  
  for channel_id in 0..=2 {
    while let Some(message) = client.receive_message(channel_id) {
      if let Ok(message) = bincode::deserialize(&message) {
        match message {
          ServerToClientMessages::PlayerConnected { id, init_data } => {
            add_net_plr.send(
              AddNetPlayer { client_id: id, init_data }
            );
          }

          ServerToClientMessages::InitData { 
            self_init, 
            player_init, 
            chat_messages 
          } => {
            //Apply self_init
            commands.entity(main_plr.0).insert(Username(self_init.username));
            main_plr.1.translation = self_init.position;

            //Apply player_init
            for (client_id, init_data) in player_init {
              add_net_plr.send(
                AddNetPlayer { client_id, init_data }
              );
            }

            //Apply chat_messages
            if chat.0.len() > 0 {
              chat.0.extend_from_slice(&chat_messages[..]);
            } else {
              chat.0 = chat_messages;
            }
          },

          ServerToClientMessages::PlayerSync { id, new_pos } => {
            if let Some(net_plr_entity) = lobby.players.get(&id) {
              net_plr_trans.get_mut(*net_plr_entity).unwrap().translation = new_pos;
            }
          }

          ServerToClientMessages::ChunkData { data, position } => {
            let position = ChunkPosition(position.0, position.1);
            info!("Chunk {:?} - Received", position);
            let task = pool.spawn(async move {
              ChunkDataComponent((data).into())
            });
            commands.spawn()
              .insert(position)
              .insert(Chunk)
              .insert(DecompressTask(task));
          },

          ServerToClientMessages::ChatMessage { message: chat_message } => { 
            chat.0.push(chat_message);
          },
          _ => warn!("Unhandled message type")
        }
      }
    }
  }
}

fn add_net_player_event_handler(
  mut commands: Commands,
  mut lobby: ResMut<Lobby>,
  mut new_players: EventReader<AddNetPlayer>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for event in new_players.iter() {
    //TODO add username floaty thing
    assert!(
      !lobby.players.contains_key(&event.client_id), 
      "[FUCK] player already added, server must be drunk"
    );
    let entity = commands.spawn()
      .insert(NetPlayer)
      .insert(Player)
      .insert(Username(event.init_data.username.clone()))
      .insert_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(event.init_data.position),
        ..default()
      }).id();
    lobby.players.insert(event.client_id, entity);
  }
}

pub fn request_chunks(
  mut events: EventReader<RequestChunk>,
  mut client: ResMut<RenetClient>,
) {
  if !client.is_connected() { return; }
  for RequestChunk(x, y) in events.iter() {
    info!("Chunk {},{} - Requested", x, y);
    client.send_message(
      CHANNEL_RELIABLE, 
      bincode::serialize(
        &ClientToServerMessages::ChunkRequest { x: *x, y: *y }
      ).unwrap()
    );
  }
}

pub fn chat_send(
  mut events: EventReader<RequestNetChatSend>,
  mut client: ResMut<RenetClient>,
) {
  for msg in events.iter() {
    client.send_message(
      CHANNEL_RELIABLE, 
      bincode::serialize(
        &ClientToServerMessages::ChatMessage{ message: msg.0.clone() }
      ).unwrap()
    );
  }
}

pub fn sync_player(
  mut client: ResMut<RenetClient>,
  player: Query<&GlobalTransform, (With<MainPlayer>, Changed<GlobalTransform>)>
) {
  let player = player.single();
  client.send_message(
    CHANNEL_UNRELIABLE, 
    bincode::serialize(&ClientToServerMessages::PlayerMove {
      new_pos: player.translation
    }).unwrap()
  );
}

pub fn apply_decompress_tasks(
  mut commands: Commands,
  mut query: Query<(Entity, &mut DecompressTask, &ChunkPosition)>,
) {
  //TODO Update chunks instead of duplicating!
  query.for_each_mut(|(entity, mut task, position)| {
    if let Some(chunk) = future::block_on(future::poll_once(&mut task.0)) {
      commands.entity(entity)
        .remove::<DecompressTask>()
        .insert(chunk);
        info!("Chunk {:?} - Decompressed", position);
    }
  });
}

const VIS_T: usize = 200;
fn renet_visualizer_create(
  mut commands: Commands
) {
  commands.insert_resource(RenetClientVisualizer::<VIS_T>::new(RenetVisualizerStyle::default()))
}
fn renet_visualizer_update(
  mut egui_context: ResMut<EguiContext>,
  visualizer: Option<ResMut<RenetClientVisualizer<VIS_T>>>,
  client: Option<Res<RenetClient>>
) {
  if client.is_some() && visualizer.is_some() {
    let (
      mut visualizer, 
      client
    ) = (visualizer.unwrap(), client.unwrap());
    visualizer.add_network_info(client.network_info());
    visualizer.show_window(egui_context.ctx_mut());
  }
}

fn disconnect(
  mut commands: Commands,
  mut client: ResMut<RenetClient>
) {
  if client.is_connected() {  
    client.disconnect();
  }
  commands.remove_resource::<Lobby>();
  commands.remove_resource::<RenetClient>();
  commands.remove_resource::<RenetClientVisualizer<VIS_T>>();
}

fn disconnect_on_exit_system(
  exit: EventReader<bevy::app::AppExit>,
  mut client: ResMut<RenetClient>,
) {
  if client.is_connected() && !exit.is_empty() {
    client.disconnect();
  }
}

pub struct NetworkingPlugin;
impl Plugin for NetworkingPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<RequestNetChatSend>();
    app.add_event::<RequestChunk>();
    app.add_event::<AddNetPlayer>();

    app.add_plugin(RenetClientPlugin);

    app.add_enter_system_set(
      GameState::Connecting,
      SystemSet::new()
        .with_system(create_renet_client)
        .with_system(renet_visualizer_create)
    );

    app.add_system_set(
      ConditionSet::new()
        .run_in_state(GameState::Connecting)
        .with_system(update_state_to_ingame)
        .into()
    );

    app.add_system_set(
      ConditionSet::new()
        .label("NetLoop")
        .run_in_state(GameState::InGame)
        .run_if(run_if_client_conected)
        .with_system(
          handle_incoming_stuff
            .chain(add_net_player_event_handler)
        )
        .with_system(request_chunks)
        .with_system(chat_send)
        .with_system(apply_decompress_tasks)
        .with_system(sync_player)
        .into()
    );

    app.add_system_set(
      ConditionSet::new()
        .run_if(run_if_client_conected)
        .with_system(disconnect_on_exit_system)
        .with_system(renet_visualizer_update)
        .into()
    );
    
    app.add_exit_system(GameState::InGame, disconnect);

  }
}
