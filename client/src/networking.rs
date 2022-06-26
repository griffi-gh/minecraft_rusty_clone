use bevy::prelude::*;
use bevy::tasks::{Task, AsyncComputeTaskPool};
use bevy_eventwork::{
  Network, ConnectionId,
  NetworkEvent, NetworkData, 
  EventworkPlugin,
  tcp::{TcpProvider, NetworkSettings}, 
};
use std::{
  net::{Ipv4Addr, IpAddr, SocketAddr},
  ops::Deref
};
use shared::{
  consts::PORT,
  types::{AuthData, AuthResult},
  networking::{
    register_messages_client,
    ChunkDataMessage, 
    ChunkDataRequestMessage,
    AuthMessage,
    AuthResultMessage
  }
};
use futures_lite::future;
use crate::chunk::{Chunk, ChunkPosition};
use crate::player::ChunkLocation;

const SPAWN_AREA_SIZE: i64 = 8;


#[derive(Default)]
pub struct ConnectSuccess;

pub struct RequestChunk(i64, i64);
impl From<ChunkPosition> for RequestChunk {
  fn from(from: ChunkPosition) -> Self {
    Self(from.0, from.1)
  }
}
impl From<ChunkLocation> for RequestChunk {
  fn from(from: ChunkLocation) -> Self {
    Self(from.0, from.1)
  }
}

pub fn connect(
  network: ResMut<Network<TcpProvider>>,
  settings: Res<NetworkSettings>,
  task_pool: Res<bevy::tasks::TaskPool>,
) {
  if network.has_connections() {
    network.disconnect(ConnectionId { id: 0 })
      .expect("Couldn't disconnect from server!");
  } else {
    info!("Connecting...");
    network.connect(
      SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), PORT),
      task_pool.deref(),
      &settings,
    );
  }
}

pub fn handle_network_events(
  mut new_network_events: EventReader<NetworkEvent>,
  network: Res<Network<TcpProvider>>,
) {
  for event in new_network_events.iter() {
    info!("Received event");
    match event {
      NetworkEvent::Connected(_) => {
        info!("Connected! Authenticating..");
        
        network.send_message(
          ConnectionId { id: 0 }, 
          AuthMessage(AuthData::from_name("TestPlayer".into()))
        ).unwrap();
      }
      NetworkEvent::Disconnected(_) => {
        info!("Disconnected!");
      }
      NetworkEvent::Error(err) => {
        error!("Network error: {}", err);
        panic!();
      }
    }
  }
}

fn handle_auth_result(
  mut net_events: EventReader<NetworkData<AuthResultMessage>>,
  mut ev_connect: EventWriter<ConnectSuccess>,
  //mut ev_reqest: EventWriter<RequestChunk>,
) {
  for event in net_events.iter() {
    match &event.0 {
      AuthResult::Ok() => {
        info!("Auth OK!");
        ev_connect.send_default();
        //info!("Spawn area size {0}x{0}; fetching chunks", SPAWN_AREA_SIZE);
        //for x in 0..SPAWN_AREA_SIZE {
        //  for y in 0..SPAWN_AREA_SIZE {
        //    ev_reqest.send(RequestChunk(x, y));
        //  }
        //}
      }
      AuthResult::Error(error) => {
        error!("Auth error: {}", error);
        panic!();
      }
    }
  }
}

#[derive(Component)]
pub struct DecompressTask(Task<Chunk>);

pub fn handle_incoming_chunks (
  mut commands: Commands,
  mut new_messages: EventReader<NetworkData<ChunkDataMessage>>,
  pool: Res<AsyncComputeTaskPool>
) {
  for new_message in new_messages.iter() {
    let new_pos = ChunkPosition(new_message.x, new_message.y);
    info!("Received chunk: {:?}", new_pos);
    info!("Starting async task");

    let data = new_message.data.clone();

    let task = pool.spawn(async move {
      info!("Decompressing chunk {:?}...", new_pos);
      Chunk((data).into())
    });

    commands.spawn()
      .insert(new_pos)
      .insert(DecompressTask(task));
  }
}

pub fn apply_decompress_tasks(
  mut commands: Commands,
  mut query: Query<(Entity, &mut DecompressTask), With<ChunkPosition>>,
) {
  //TODO: Update instead of duplicating!
  query.for_each_mut(|(entity, mut task)| {
    if let Some(chunk) = future::block_on(future::poll_once(&mut task.0)) {
      commands.entity(entity)
        .remove::<DecompressTask>()
        .insert(chunk);
      info!("Chunk ready");
    }
  });
}

pub fn request_chunks(
  network: Res<Network<TcpProvider>>,
  mut events: EventReader<RequestChunk>,
) {
  for RequestChunk(x, y) in events.iter() {
    assert!(network.has_connections(), "Not connected yet");
    info!("Reqesting chunk at coords: {},{}", x, y);
    match network.send_message(
      ConnectionId { id: 0 },
      ChunkDataRequestMessage {
        x: *x,
        y: *y
      },
    ) {
      Err(error) => {
        error!("Communication error: {}", error);
        panic!();
      },
      Ok(_) => {}
    }
  }
}

pub struct NetworkingPlugin;
impl Plugin for NetworkingPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(NetworkSettings::default());
    app.add_plugin(EventworkPlugin::<
      TcpProvider,
      bevy::tasks::TaskPool,
    >::default());
    register_messages_client(app);

    app.add_event::<ConnectSuccess>();
    app.add_event::<RequestChunk>();

    app.add_startup_system(connect);

    app.add_system(handle_network_events);
    app.add_system(handle_auth_result);
    app.add_system(request_chunks);
    app.add_system(handle_incoming_chunks);
    app.add_system(apply_decompress_tasks);
  }
}
