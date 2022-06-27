use bevy::prelude::*;
use bevy_renet::{
  renet::{
    ConnectToken, 
    RenetConnectionConfig, 
    RenetServer, 
    ServerConfig,
    ServerEvent, 
    RenetError,
    NETCODE_KEY_BYTES
  },
  RenetServerPlugin,
};
use rand::{
  rngs::StdRng,
  SeedableRng as _,
  Rng as _
};
use std::{
  net::{UdpSocket, SocketAddr}, 
  time::SystemTime,
};
use shared::consts::{PROTOCOL_ID,MAX_CLIENTS};
use crate::Args;

fn create_renet_server(
  mut commands: Commands,
  args: Res<Args>,
) {
  //Get server address
  let public_addr = SocketAddr::new(args.ip, args.port);
  info!("Server address: {}", public_addr);

  //Bind a udp socket
  let socket = UdpSocket::bind(public_addr).expect("Failed to bind UdpSocket");
  
  //Generate private key
  let private_key: [u8; NETCODE_KEY_BYTES] = StdRng::from_entropy().gen();

  //Create connection config stuff
  let connection_config = RenetConnectionConfig::default();
  let server_config = ServerConfig::new(
    MAX_CLIENTS, PROTOCOL_ID, public_addr, private_key
  );

  //Get current time
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

  //Create a Renet server
  let server = RenetServer::new(
    current_time, server_config, connection_config, socket
  ).expect("Failed to create Renet server");

  //Insert the server resource
  commands.insert_resource(server);
}

fn panic_on_error_system(mut renet_error: EventReader<RenetError>) {
  for error in renet_error.iter() {
    panic!("{}", error);
  }
}

pub struct ServerPlugin;
impl Plugin for ServerPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugin(RenetServerPlugin);
    app.add_startup_system(create_renet_server);
  }
}
