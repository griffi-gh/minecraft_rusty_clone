use bevy::prelude::*;
use bevy::tasks::{Task, AsyncComputeTaskPool};
use bevy_renet::{
  renet::{
    RenetServer, 
    ServerConfig,
    ServerEvent, 
    NETCODE_KEY_BYTES
  },
  RenetServerPlugin
};
use futures_lite::future;
use rand::{
  rngs::StdRng,
  //Traits
  SeedableRng as _,
  Rng as _
};
use bincode;
use std::{
  net::{UdpSocket, SocketAddr}, 
  time::SystemTime,
};
use shared::{
  blocks::BlockTypeManager,
  messages::{ServerToClientMessages, ClientToServerMessages, renet_connection_config},
  consts::{ 
    PROTOCOL_ID, MAX_CLIENTS, CHANNEL_RELIABLE, CHANNEL_UNRELIABLE 
  },
  utils::{
    print_on_renet_error_system,
    check_username,
  },
  types::{
    AuthUserData,
    PlayerInitData,
    ChatMessage,
    Lobby,
    Username
  },
};
use crate::{
  worldgen::generate as generate_chunk,
  Args,
};

pub struct PrivateKey(pub [u8; NETCODE_KEY_BYTES]);

#[derive(Component, Debug, Clone, Copy)]
pub struct Player { pub id: u64 }

pub struct SendSysMessageEvt(pub String);

fn create_renet_server(
  mut commands: Commands, 
  args: Res<Args>,
  key: Res<PrivateKey>
) {
  //Get server address
  let public_addr = SocketAddr::new(args.ip, args.port_server);
  info!("Server Address: {}", public_addr);

  //Bind a udp socket
  let socket = UdpSocket::bind(public_addr).expect("Failed to bind UdpSocket");
  
  //Create connection config stuff
  let connection_config = renet_connection_config();
  let server_config = ServerConfig::new(
    MAX_CLIENTS, PROTOCOL_ID, public_addr, key.0
  );

  //Get current time
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

  //Create a Renet server
  let server = RenetServer::new(
    current_time, server_config, connection_config, socket
  ).expect("Failed to create Renet server");

  //Insert the server resource
  commands.insert_resource(server);

  info!("Server started");
}


//TODO Maybe separate into multiple systems
fn server_update_system(
  mut server_events: EventReader<ServerEvent>,
  mut commands: Commands,
  mut lobby: ResMut<Lobby>,
  mut server: ResMut<RenetServer>,
  mut sys_msg: EventWriter<SendSysMessageEvt>,
  players: Query<(&Player, &Username, &GlobalTransform)>
) {
  'evt_loop: for event in server_events.iter() {
    match event {
      ServerEvent::ClientConnected(id, user_data) => {
        let user_data: AuthUserData = {
          let end = (user_data[0] + 1) as usize;
          let slice = &user_data[1..end];
          match bincode::deserialize::<AuthUserData>(slice) {
            Err(_) => {
              warn!("Some asshole tried to send a corrupted user data object");
              server.disconnect(*id);
              continue 'evt_loop;
            },
            Ok(parsed) => {
              if check_username(parsed.username.as_str()).is_err() {
                warn!("Oops username validation failed");
                server.disconnect(*id);
                continue 'evt_loop;
              }
              parsed
            }
          }
        };
        let AuthUserData{ username, .. } = user_data;

        info!("Player {} with username {} connected.", id, &username);

        //Spawn Player Entity
        let plr_transform = Transform::from_xyz(0., 60., 0.);
        let player_entity = commands.spawn()
          .insert_bundle(TransformBundle::from_transform(plr_transform))
          .insert(Player { id: *id })
          .insert(Username(username.clone()))
          .id();
        
        //Insert it into Lobby
        lobby.players.insert(*id, player_entity);

        //Send init data
        server.send_message(
          *id, CHANNEL_RELIABLE, 
          bincode::serialize(&ServerToClientMessages::InitData {
            self_init: PlayerInitData { 
              position: plr_transform.translation,
              username: username.clone()
            },
            player_init: {
              let mut player_init = Vec::new();
              for (player, name, transform) in players.iter() {
                if player.id == *id { continue }
                player_init.push((
                  player.id, 
                  PlayerInitData {
                    position: transform.translation,
                    username: name.0.clone()
                  }
                ));
              }
              player_init
            },
            chat_messages: Vec::new(), //TODO sync chat
          }).unwrap()
        );

        //Send player connected message
        server.broadcast_message_except(
          *id,
          CHANNEL_RELIABLE, 
          bincode::serialize(&ServerToClientMessages::PlayerConnected { 
            id: *id,
            init_data: PlayerInitData {
              position: plr_transform.translation,
              username: username.clone()
            }
          }).unwrap()
        );

        //Send chat message
        sys_msg.send(SendSysMessageEvt(
          format!("Player {} connected.", &username)
        ));
      }
      
      ServerEvent::ClientDisconnected(id) => {
        info!("Player {} disconnected.", id);

        //Remove the player and get the username
        let mut username = default();
        if let Some(player_entity) = lobby.players.remove(id) {
          username = players.get(player_entity).unwrap().1.0.clone();
          commands.entity(player_entity).despawn();
        }

        //Broadcast disconnect message
        server.broadcast_message_except(
          *id, CHANNEL_RELIABLE, 
          bincode::serialize(&ServerToClientMessages::PlayerDisconnected { id: *id }).unwrap()
        );

        //Send system chat message
        sys_msg.send(SendSysMessageEvt(
          format!("Player {} disconnected.", &username)
        ));
      }
    }
  }
}

#[derive(Component)]
struct ChunkGenTask{
  pub task: Task<Vec<u8>>,
  pub client_id: u64,
}

fn process_chunk_gen_tasks(
  mut commands: Commands,
  mut server: ResMut<RenetServer>,
  mut tasks: Query<(Entity, &mut ChunkGenTask)>
) {
  for (entity, mut task) in tasks.iter_mut() {
    if let Some(message) = future::block_on(future::poll_once(&mut task.task)) {
      info!("Chunk size: {} bytes", message.len());
      server.send_message(task.client_id, CHANNEL_UNRELIABLE, message);
      commands.entity(entity).despawn();
    }; 
  }
}

fn send_system_messages(
  mut events: EventReader<SendSysMessageEvt>,
  mut server: ResMut<RenetServer>,
) {
  for evt in events.iter() {
    server.broadcast_message(
      CHANNEL_RELIABLE, 
      bincode::serialize(&ServerToClientMessages::ChatMessage { 
        message: ChatMessage {
          message: evt.0.clone(),
          from: "[SERVER]".into(),
          timestamp: SystemTime::now(),
          is_system: true
        }
      }).unwrap()
    );
  }
}

//TODO!!! Server: Separate into multiple systems
fn handle_incoming_stuff(
  mut commands: Commands,
  mut server: ResMut<RenetServer>,
  pool: Res<AsyncComputeTaskPool>,
  blocks: Res<BlockTypeManager>,
  lobby: Res<Lobby>,
  mut players: Query<(&mut Transform, &Username), With<Player>>,
) {
  for client_id in server.clients_id() {
    for channel_id in 0..=2 {
      while let Some(message) = server.receive_message(client_id, channel_id) {
        if let Ok(message) = bincode::deserialize(&message) {
          match message {

            ClientToServerMessages::ChunkRequest {x, y} => {
              info!("Chunk request {} {}", x, y);
              let blocks_uwu = blocks.clone();
              let task = pool.spawn(async move {
                bincode::serialize(&ServerToClientMessages::ChunkData { 
                  data: generate_chunk(x, y, &blocks_uwu).into(), 
                  position: (x, y)
                }).unwrap()
              });
              commands.spawn().insert(ChunkGenTask{ task, client_id });
            },

            ClientToServerMessages::ChatMessage { message } => {
              server.broadcast_message_except(
                client_id, CHANNEL_RELIABLE, 
                bincode::serialize(&ServerToClientMessages::ChatMessage { 
                  message: ChatMessage {
                    message,
                    from: players.get(*lobby.players.get(&client_id).unwrap()).unwrap().1.0.clone(),
                    timestamp: SystemTime::now(),
                    is_system: false
                  }
                }).unwrap()
              );
            },

            ClientToServerMessages::PlayerMove { new_pos } => {
              if let Some(entity) = lobby.players.get(&client_id) {
                let transform: &mut Transform = &mut players.get_mut(*entity).unwrap().0;
                transform.translation = new_pos;
                server.broadcast_message_except(
                  client_id, CHANNEL_UNRELIABLE, 
                  bincode::serialize(&ServerToClientMessages::PlayerSync {
                    id: client_id, new_pos 
                  }).unwrap()
                );
              }
            },

            _ => warn!("Unhandled message type")
          }
        }
      }
    }
  }
}

pub struct ServerPlugin;
impl Plugin for ServerPlugin {
  fn build(&self, app: &mut App) {
    //Generate private key
    app.add_event::<SendSysMessageEvt>();
    app.init_resource::<Lobby>();
    app.insert_resource(PrivateKey(StdRng::from_entropy().gen()));
    app.add_plugin(RenetServerPlugin);
    app.add_startup_system(create_renet_server);
    app.add_system(print_on_renet_error_system);
    app.add_system(server_update_system);
    app.add_system(handle_incoming_stuff);
    app.add_system(process_chunk_gen_tasks);
    app.add_system(send_system_messages);
  }
}
