use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_renet::{
  RenetClientPlugin,
  run_if_client_conected,
  renet::{
    RenetClient,
    ConnectToken
  }
};
use renet_visualizer::{RenetClientVisualizer, RenetVisualizerStyle};
use bevy_egui::EguiContext;
use futures_lite::future;
use bincode;
use serde_json::Value as JsonValue;
use reqwest;
use base64;
use std::{
  time::{SystemTime},
  net::{IpAddr, SocketAddr, UdpSocket},
  io::Cursor
};
use shared::{
  messages::{ClientMessages, ServerMessages, renet_connection_config},
  consts::{
    CHANNEL_RELIABLE, CHANNEL_UNRELIABLE, DEFAULT_PORT
  },
};
use crate::{
  chat::ChatMessages,
  player::ChunkLocation,
  chunk::{Chunk, ChunkPosition},
  player::MainPlayer
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

#[derive(Component)]
pub struct DecompressTask(pub Task<Chunk>);

fn create_renet_client(
  mut commands: Commands
) {
  let server_ip: IpAddr = [127,0,0,1].into();
  let api_url = format!("http://{}:{}", server_ip.to_string(), DEFAULT_PORT);

  //Get connection data
  let conn_data: JsonValue = {
    let res = reqwest::blocking::get(format!("{}/connect", api_url)).expect("Failed to get the connection token");
    let res_bytes = &res.bytes().unwrap()[..];
    serde_json::from_slice(res_bytes).unwrap()
  };

  //Parse it
  let (connect_token, client_id) = (
    {
      let token_base64 = conn_data["token"].as_str().expect("No token in response");
      let token_bytes = base64::decode(token_base64).expect("Invalid token Base64");
      ConnectToken::read(&mut Cursor::new(&token_bytes)).unwrap()
    },
    conn_data["client_id"].as_u64().expect("No Client ID in response"),
    // conn_data["port"].as_u64().expect("No port in response") as u16,
  );
  
  //Bind socket
  let socket = UdpSocket::bind(SocketAddr::new(server_ip, 0)).unwrap();

  //Create config things
  let connection_config = renet_connection_config();
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

  let client = RenetClient::new(current_time, socket, client_id, connect_token, connection_config).unwrap();
  commands.insert_resource(client);

  info!("Client started");
}

//TODO!!! Client: Separate into multiple systems
fn handle_incoming_stuff(
  mut commands: Commands,
  mut client: ResMut<RenetClient>,
  pool: Res<AsyncComputeTaskPool>,
  mut chat: ResMut<ChatMessages>,
) {
  if !client.is_connected() { return; }
  for channel_id in 0..=2 {
    while let Some(message) = client.receive_message(channel_id) {
      if let Ok(message) = bincode::deserialize(&message) {
        match message {
          ServerMessages::ChunkData { data, position } => {
            let position = ChunkPosition(position.0, position.1);
            info!("Chunk {:?} - Received", position);
            let task = pool.spawn(async move {
              Chunk((data).into())
            });
            commands.spawn()
              .insert(position)
              .insert(DecompressTask(task));
          },
          ServerMessages::ChatMessage { message: chat_message } => { 
            chat.0.push(chat_message);
          },
          _ => warn!("Unhandled message type")
        }
      }
    }
  }
}

//TODO maybe move request_chunks and apply_decompress_tasks?

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
        &ClientMessages::ChunkRequest { x: *x, y: *y }
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
        &ClientMessages::ChatMessage{ message: msg.0.clone() }
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
    bincode::serialize(&ClientMessages::PlayerSync {
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
  mut visualizer: ResMut<RenetClientVisualizer::<VIS_T>>,
  client: Res<RenetClient>
) {
  visualizer.add_network_info(client.network_info());
  visualizer.show_window(egui_context.ctx_mut());
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

    app.add_plugin(RenetClientPlugin);

    app.add_startup_system(create_renet_client);
    app.add_system_set(
      SystemSet::new()
        .label("NetHandler")
        .with_run_criteria(run_if_client_conected)
        .with_system(handle_incoming_stuff)
        .with_system(request_chunks)
        .with_system(chat_send)
        .with_system(apply_decompress_tasks)
        .with_system(sync_player)
    );
    app.add_startup_system(renet_visualizer_create);
    app.add_system(renet_visualizer_update);
    app.add_system(disconnect_on_exit_system);
  }
}
