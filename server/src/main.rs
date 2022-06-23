use bevy::prelude::*;
use bevy_eventwork::{
  EventworkPlugin, Network,
  ConnectionId, NetworkEvent,
  NetworkData,
  tcp::{TcpProvider, NetworkSettings}, 
};
use std::{
  net::{SocketAddr, IpAddr, Ipv4Addr}, 
  ops::Deref
};
use shared::{
  consts::PORT,
  networking::{
    register_messages_server,
    ChunkDataRequestMessage,
    ChunkDataMessage, AuthMessage, AuthResultMessage
  }, types::AuthResult
};

mod worldgen;
use worldgen::generate as generate_chunk;

#[derive(Component)]
struct Player(ConnectionId);

#[derive(Component)]
struct AuthenticatedPlayer;

#[derive(Component)]
struct PlayerName(String);

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

  app.add_system(handle_chunk_request_messages);
  app.add_system(handle_auth_request_messages);
  app.add_system(handle_chat_messages);

  app.run();
}

fn setup_networking(
  mut net: ResMut<Network<TcpProvider>>,
  settings: Res<NetworkSettings>,
  runtime: Res<bevy::tasks::TaskPool>,
) {
  let ip_address: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
  info!("Server address: {}:{}", ip_address, PORT);
  match net.listen(
    SocketAddr::new(ip_address, PORT),
    runtime.deref(),
    &settings,
  ) {
    Ok(_) => {
      info!("Started listening for new connections!");
    },
    Err(err) => {
      error!("Could not start listening: {}", err);
      panic!();
    }
  }
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

//not a system!
fn find_player(
  connection: &ConnectionId,
  players: &Query<&Player, With<AuthenticatedPlayer>>
) -> bool {
  for player in players.iter() {
    if player.0 == *connection {
      return true;
    }
  }
  false
}

fn handle_chunk_request_messages(
  mut chunk_requests: EventReader<NetworkData<ChunkDataRequestMessage>>,
  network: Res<Network<TcpProvider>>,
  auth_players: Query<&Player, With<AuthenticatedPlayer>>
) {
  for message in chunk_requests.iter() {
    let user = message.source();

    if !find_player(user, &auth_players) {
      network.disconnect(*user).unwrap();
      return;
    }

    info!("User \"{}\" requested chunk at ({}, {})", user, message.x, message.y);

    let _ = network.send_message(*user, ChunkDataMessage {
      data: generate_chunk(message.x, message.y).into(),
      x: message.x,
      y: message.y
    }).map_err(|e| error!("{}", e));
  }
}


fn handle_auth_request_messages(
  mut commands: Commands,
  mut auth_requests: EventReader<NetworkData<AuthMessage>>,
  network: Res<Network<TcpProvider>>,
  players: Query<(Entity, &Player), Without<AuthenticatedPlayer>>
) {
  for message in auth_requests.iter() {
    //TODO send "Player connected" chat message

    let user = message.source();
    
    let mut auth_result = AuthResult::Error("Already connected".into());

    for (entity, player) in players.iter() {
      if player.0 == *user {
        auth_result = AuthResult::Ok();
        commands.entity(entity)
          .insert(AuthenticatedPlayer)
          .insert(PlayerName(message.0.name.clone()));
        break;
      }
    }

    let _ = network.send_message(
      *user,
      AuthResultMessage(auth_result)
    ).map_err(|e| error!("{}", e));

  }
}

fn handle_chat_messages() {
  //TODO
}
