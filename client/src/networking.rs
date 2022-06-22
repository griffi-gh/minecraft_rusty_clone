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
use shared::networking::{
  ChunkDataMessage, 
  ChunkDataRequestMessage,
  register_messages_client
};
use crate::chunk::{Chunk, ChunkPosition};

#[derive(Default)]
pub struct ConnectSuccess;

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
  mut ev_connect: EventWriter<ConnectSuccess>
) {
  for event in new_network_events.iter() {
    info!("Received event");
    match event {
      NetworkEvent::Connected(_) => {
        info!("Connected!");
        ev_connect.send_default();
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
    println!("Received chunk: {:?}", new_pos);
    let chunk = Chunk(new_message.data.clone().into());
    for (entity, position) in chunks.iter() {
      if *position == new_pos {
        commands.entity(entity).despawn();
      }
    }
    commands.spawn().insert(chunk).insert(new_pos);
  }
}

pub fn test_request_chunk_at_0_0(
  network: Res<Network<TcpProvider>>,
  mut ev_connect: EventReader<ConnectSuccess>,
) {
  for _ in ev_connect.iter() {
    assert!(network.has_connections(), "Not connected yet");
    info!("Received connection event, requesting chunk");
    match network.send_message(
      ConnectionId { id: 0 },
      ChunkDataRequestMessage {
        x: 0, y: 0
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

    app.add_startup_system(connect);
    app.add_event::<ConnectSuccess>();

    app.add_system(test_request_chunk_at_0_0);
    app.add_system(handle_network_events);
    app.add_system(handle_incoming_chunks);
  }
}
