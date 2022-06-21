use bevy::prelude::*;
use bevy_eventwork::{
  Network, ConnectionId, NetworkEvent,
  tcp::{TcpProvider, NetworkSettings}, 
};
use std::{
  net::{Ipv4Addr, IpAddr, SocketAddr},
  ops::Deref
};

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
) {
  for event in new_network_events.iter() {
    info!("Received event");
    match event {
      NetworkEvent::Connected(_) => {
        info!("Connected!");
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
