use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_renet::{
  RenetClientPlugin,
  run_if_client_conected,
  renet::{
    RenetClient,
    RenetConnectionConfig,
    ConnectToken
  }
};
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

  let server_addr = SocketAddr::new(server_ip, 0);

  //Bind socket
  let socket = UdpSocket::bind(server_addr).unwrap();

  //Create config things
  let connection_config = RenetConnectionConfig::default();
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

  let client = RenetClient::new(current_time, socket, client_id, connect_token, connection_config).unwrap();
  commands.insert_resource(client);

  info!("Client started");
}

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

pub struct NetworkingPlugin;
impl Plugin for NetworkingPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<RequestChunk>();
    app.add_plugin(RenetClientPlugin);
    app.add_startup_system(create_renet_client);
    app.add_system_set(
      SystemSet::new()
        .label("NetHandler")
        .with_run_criteria(run_if_client_conected)
        .with_system(handle_incoming_stuff)
        .with_system(request_chunks)
        .with_system(apply_decompress_tasks)
    );
  }
}
