use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_renet::{
  RenetClientPlugin,
  renet::{
    RenetClient,
    RenetConnectionConfig,
    ConnectToken
  }
};
use futures_lite::future;
use bincode;
use reqwest;
use base64;
use std::{
  time::{SystemTime},
  net::{SocketAddr, UdpSocket},
  io::Cursor
};
use shared::{
  messages::{ClientMessages, ServerMessages},
  consts::{
    CHANNEL_BLOCK, CHANNEL_RELIABLE, 
    CHANNEL_UNRELIABLE, DEFAULT_PORT
  },
};
use crate::{
  chat::ChatMessages,
  player::ChunkLocation,
  chunk::{Chunk, ChunkPosition},
};


#[derive(Clone, Copy, Debug)]
pub struct RequestChunk(i64, i64);
impl From<ChunkPosition> for RequestChunk {
  fn from(from: ChunkPosition) -> Self { Self(from.0, from.1) }
}
impl From<ChunkLocation> for RequestChunk {
  fn from(from: ChunkLocation) -> Self { Self(from.0, from.1) }
}

#[derive(Component)]
pub struct DecompressTask(Task<Chunk>);

fn new_renet_client(
  mut commands: Commands
) {
  let server_addr = SocketAddr::new([127,0,0,1].into(), DEFAULT_PORT);
  let url = format!("{}:{}", server_addr.ip().to_string(), DEFAULT_PORT + 1);
  let socket = UdpSocket::bind(server_addr).unwrap();

  //Create config things
  let connection_config = RenetConnectionConfig::default();
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
  let client_id = current_time.as_millis() as u64;

  //Get token
  let res = reqwest::blocking::get(format!("{}/connect", url)).expect("Failed to get the connection token");
  let res_bytes = res.bytes().unwrap();
  let token_bytes = base64::decode(res_bytes).unwrap();
  let token = ConnectToken::read(&mut Cursor::new(&token_bytes)).unwrap();

  RenetClient::new(current_time, socket, client_id, token, connection_config).unwrap();
}

fn handle_incoming_stuff(
  mut commands: Commands,
  mut client: ResMut<RenetClient>,
  pool: Res<AsyncComputeTaskPool>,
  mut messages: ResMut<ChatMessages>,
) {
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
          ServerMessages::ChatMessage { message } => { 
            messages.0.push(message);
          },
          _ => warn!("Unhandled message type")
        }
      }
    }
  }
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

pub fn request_chunks(
  mut events: EventReader<RequestChunk>,
  mut client: ResMut<RenetClient>,
) {
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

pub struct NetworkingPlugin;
impl Plugin for NetworkingPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<RequestChunk>();
    app.add_plugin(RenetClientPlugin);
    app.add_system(request_chunks);
    app.add_system(handle_incoming_stuff);
    app.add_system(apply_decompress_tasks);
  }
}
