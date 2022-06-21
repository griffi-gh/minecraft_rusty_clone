use bevy::prelude::*;
use bevy_eventwork::{
  EventworkPlugin, Network,
  ConnectionId, NetworkEvent,
  tcp::{TcpProvider, NetworkSettings}, 
};
use std::{
  net::{SocketAddr, IpAddr, Ipv4Addr}, 
  ops::Deref
};
use shared::networking::register_messages_server;

#[derive(Component)]
struct Player(ConnectionId);

fn main() {
  let mut app = App::new();

  app.add_plugins(MinimalPlugins);
  app.add_plugin(bevy::log::LogPlugin);
  
  app.insert_resource(bevy::tasks::TaskPoolBuilder::new().build());

  app.insert_resource(NetworkSettings::default());
  app.add_plugin(EventworkPlugin::<
    TcpProvider,
    bevy::tasks::TaskPool,
  >::default());
  register_messages_server(&mut app);
  
  app.add_startup_system(setup_networking);
  app.add_system(handle_network_events);

  app.run();
}

fn setup_networking(
  mut net: ResMut<Network<TcpProvider>>,
  settings: Res<NetworkSettings>,
  runtime: Res<bevy::tasks::TaskPool>,
) {
  let ip_address: IpAddr = "127.0.0.1".parse().expect("Could not parse ip address");

  info!("Address of the server: {}", ip_address);

  match net.listen(
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    runtime.deref(),
    &settings,
  ) {
    Ok(_) => (),
    Err(err) => {
      error!("Could not start listening: {}", err);
      panic!();
    }
  }

  info!("Started listening for new connections!");
}

fn handle_network_events(
  mut commands: Commands,
  mut network_events: EventReader<NetworkEvent>,
  players: Query<(Entity, &Player)>
  //network: Res<Network<TcpProvider>>,
) {
  for event in network_events.iter() {
    match event {
      NetworkEvent::Connected(conn_id) => {
        commands.spawn_bundle((Player(*conn_id),));
        info!("New player connected: {}", conn_id);
      }
      NetworkEvent::Disconnected(conn_id) => {
        info!("Someone is leaving...");
        for (entity, player) in players.iter() {
          if player.0 == *conn_id {
            commands.entity(entity).despawn();
            break;
          }
        }
        info!("Player disconnected: {}", conn_id);
      }
      NetworkEvent::Error(err) => {
        error!("Network error: {}", err);
        panic!();
      }
    }
  }
}
