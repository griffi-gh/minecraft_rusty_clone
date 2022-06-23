use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_eventwork::{
  NetworkData, NetworkEvent,
  ConnectionId,Network,
  tcp::TcpProvider,
};
use shared::{
  consts::PROTOCOL_VERSION,
  types::AuthResult,
  networking::{AuthMessage, AuthResultMessage}
};
use crate::{Player, PlayerName, AuthenticatedPlayer};

fn handle_auth_request_messages(
  mut commands: Commands,
  mut auth_requests: EventReader<NetworkData<AuthMessage>>,
  network: Res<Network<TcpProvider>>,
  players: Query<(Entity, &Player), Without<AuthenticatedPlayer>>,
  mut auth_map: ResMut<AuthenticatedPlayerMap>
) {
  for message in auth_requests.iter() {
    //TODO send "Player connected" chat message

    let user = message.source();
    
    let respond = |result| {
      let _ = network.send_message(
        *user,
        AuthResultMessage(result)
      ).map_err(|e| error!("{}", e));
    };
    
    if message.0.protocol_version != PROTOCOL_VERSION {
      let client_ver = message.0.protocol_version;
      let server_ver = PROTOCOL_VERSION;
      respond(AuthResult::Error(format!(
        "{} (Client: v{}; Server: v{})", 
        if client_ver < server_ver {
          "Outdated game client"
        } else {
          "Outdated server version"
        },
        client_ver, server_ver
      )));
      return;
    }

    for (entity, player) in players.iter() {
      if player.0 == *user {
        commands.entity(entity)
          .insert(AuthenticatedPlayer)
          .insert(PlayerName(message.0.name.clone()));
        auth_map.0.insert(*user);
        respond(AuthResult::Ok());
        return;
      }
    }

    respond(AuthResult::Error("Already authenticated".into()));
  }
}

fn handle_disconnect_event(
  mut network_events: EventReader<NetworkEvent>,
  mut auth_map: ResMut<AuthenticatedPlayerMap>
) {
  for event in network_events.iter() {
    match event {
      NetworkEvent::Disconnected(conn_id) => {
        if auth_map.0.remove(conn_id) {
          info!("Player deauthenticated: {}", conn_id);
        } else {
          warn!("Player left before authenticating: {}", conn_id);
        }
      }
      _ => {} // I don't care about the rest
    }
  }
}

#[derive(Default, Debug)]
pub struct AuthenticatedPlayerMap(HashSet<ConnectionId>);

pub struct ServerPassword(Option<String>);

pub struct AuthPlugin;
impl Plugin for AuthPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(AuthenticatedPlayerMap::default());
    app.add_system(
      handle_auth_request_messages
        .chain(handle_disconnect_event)
    );
  }
}
