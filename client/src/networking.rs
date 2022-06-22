use bevy::prelude::*;
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
  networking::{
    ChunkDataMessage, 
    ChunkDataRequestMessage,
    register_messages_client
  },
  types::{
    ChunkData,
    CompressedChunkData
  }
};
use crate::chunk::{Chunk, ChunkPosition};

#[derive(Default)]
pub struct ConnectSuccess;

pub struct RequestChunk(i64, i64);

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
      SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
      task_pool.deref(),
      &settings,
    );
  }
}

pub fn handle_network_events(
  mut new_network_events: EventReader<NetworkEvent>,
  mut ev_connect: EventWriter<ConnectSuccess>,
  mut ev_reqest: EventWriter<RequestChunk>
) {
  for event in new_network_events.iter() {
    info!("Received event");
    match event {
      NetworkEvent::Connected(_) => {
        info!("Connected!");
        ev_connect.send_default();
        for x in 0..8 {
          for y in 0..8 {
            ev_reqest.send(RequestChunk(x, y));
          }
        }
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

pub fn handle_incoming_chunks(
  mut commands: Commands,
  mut new_messages: EventReader<NetworkData<ChunkDataMessage>>,
  chunks: Query<(Entity, &ChunkPosition), With<Chunk>>
) {
  for new_message in new_messages.iter() {
    let new_pos = ChunkPosition(new_message.x, new_message.y);
    info!("Received chunk: {:?}", new_pos);
    info!("Decompressing...");
    let data: ChunkData = (&new_message.data).into();
    let chunk = Chunk(data);
    info!("Adding...");
    for (entity, position) in chunks.iter() {
      if *position == new_pos {
        commands.entity(entity).despawn();
      }
    }
    commands.spawn().insert(chunk).insert(new_pos);
    info!("Done");
  }
}

pub fn request_chunks(
  network: Res<Network<TcpProvider>>,
  mut events: EventReader<RequestChunk>,
) {
  for RequestChunk(x, y) in events.iter() {
    assert!(network.has_connections(), "Not connected yet");
    info!("Received connection event, requesting chunk");
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

    app.add_system(
      handle_network_events
        .chain(request_chunks)
        .chain(handle_incoming_chunks)
    );
  }
}
