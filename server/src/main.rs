use bevy::prelude::*;
use bevy::{
  app::ScheduleRunnerSettings,
  log::LogPlugin,
  transform::TransformPlugin,
  hierarchy::HierarchyPlugin,
  scene::ScenePlugin
};
use clap::Parser;
use std::{
  net::{IpAddr, Ipv4Addr}, 
  time::Duration,
};
use shared::{
  consts::DEFAULT_PORT,
  blocks::BlockManagerPlugin
};

mod server;
mod worldgen;
mod chat;

use server::ServerPlugin;
use worldgen::generate as generate_chunk;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
  #[clap(short, long, value_parser, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
  ip: IpAddr,

  #[clap(short, long, value_parser, default_value_t = DEFAULT_PORT)]
  port: u16,
}

fn main() {
  let mut app = App::new();

  app.insert_resource(Args::parse());

  app.add_plugins(MinimalPlugins);
  app.add_plugin(LogPlugin);
  app.add_plugin(TransformPlugin);
  app.add_plugin(HierarchyPlugin);

  app.insert_resource(bevy::tasks::TaskPoolBuilder::new().build());
  app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(1./60.)));

  app.add_plugin(BlockManagerPlugin);
  app.add_plugin(ServerPlugin);

  app.run();
}


// fn handle_network_events(
//   mut commands: Commands,
//   mut network_events: EventReader<NetworkEvent>,
//   players: Query<(Entity, &Player)>
// ) {
//   for event in network_events.iter() {
//     match event {
//       NetworkEvent::Connected(conn_id) => {
//         commands.spawn_bundle((Player(*conn_id),));
//         info!("New player connected: {}", conn_id);
//       }
//       NetworkEvent::Disconnected(conn_id) => {
//         info!("Someone is leaving...");
//         for (entity, player) in players.iter() {
//           if player.0 == *conn_id {
//             commands.entity(entity).despawn();
//             break;
//           }
//         }
//         info!("Player disconnected: {}", conn_id);
//       }
//       NetworkEvent::Error(err) => {
//         error!("Network error: {}", err);
//         panic!();
//       }
//     }
//   }
// }

// fn find_player(
//   connection: &ConnectionId,
//   players: &Query<&Player, With<AuthenticatedPlayer>>
// ) -> bool {
//   for player in players.iter() {
//     if player.0 == *connection {
//       return true;
//     }
//   }
//   false
// }


// fn handle_chunk_request_messages(
//   mut chunk_requests: EventReader<NetworkData<ChunkDataRequestMessage>>,
//   network: Res<Network<TcpProvider>>,
//   auth_players: Query<&Player, With<AuthenticatedPlayer>>,
//   blocks: Res<BlockTypeManager>
// ) {
//   for message in chunk_requests.iter() {
//     let user = message.source();

//     if !find_player(user, &auth_players) {
//       network.disconnect(*user).unwrap();
//       return;
//     }

//     info!("User \"{}\" requested chunk at ({}, {})", user, message.x, message.y);

//     let _ = network.send_message(*user, ChunkDataMessage {
//       data: generate_chunk(message.x, message.y, &blocks).into(),
//       x: message.x,
//       y: message.y
//     }).map_err(|e| error!("{}", e));
//   }
// }
