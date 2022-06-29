use bevy::prelude::*;
use bevy::{
  tasks::AsyncComputeTaskPool,
  utils::HashMap
};
use bevy_renet::renet::ConnectToken;

use tokio::runtime::Runtime as TokioRuntime;
use warp::Filter;

use base64;

use serde_json::json;

use std::{
  net::SocketAddr, time::SystemTime,
};
use crate::{
  Args, server::PrivateKey
};
use shared::consts::PROTOCOL_ID;

const EXPIRE_SECONDS: u64 = 300;
const TIMEOUT_SECONDS: i32 = 15;

fn start(
  pool: Res<AsyncComputeTaskPool>,
  args: Res<Args>,
  private_key: Res<PrivateKey>
) {
  let args = args.clone();
  let private_key = private_key.0;
  pool.spawn(async move {
    let runtime = TokioRuntime::new().unwrap();
    runtime.block_on(async move {
      //=========================================================
      let root = warp::path!().map(move || {
        warp::reply::json(&json!({
          "name": "Game Server",
          "description": "no description",
          "icon": null,
          "private": false,
        }))
      });

      let connect = 
        warp::path!("connect")
        .and(warp::query::<HashMap<String, String>>())
        .map(move |_query: HashMap<String, String>| {
          //TODO Password auth
          //TODO Rate limiting
          info!("Connect token requested");

          let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
          let client_id = current_time.as_millis() as u64;
          let mut server_addresses = vec![SocketAddr::new(args.ip, args.port_server)];
          if args.ip.is_loopback() || args.ip.is_unspecified() {
            info!("Running on loopback address, allowing connections from localhost");
            let ipv4_loopback = [127,0,0,1].into();
            let ipv6_loopback = [0,0,0,0,0,0,0,1].into();
            server_addresses.push(SocketAddr::new(ipv4_loopback, args.port_server));
            server_addresses.push(SocketAddr::new(ipv6_loopback, args.port_server));
            server_addresses.push(SocketAddr::new(ipv4_loopback, 0));
            server_addresses.push(SocketAddr::new(ipv6_loopback, 0));
          }
          let mut buffer = Vec::new();
          ConnectToken::generate(
            current_time, PROTOCOL_ID, EXPIRE_SECONDS, client_id, TIMEOUT_SECONDS, 
            server_addresses.clone(), None, &private_key
          ).expect("Failed to generate the token").write(&mut buffer).unwrap();

          warp::reply::json(&json!({
            "success": true,
            "token": base64::encode(&buffer),
            "port": server_addresses[0].port(),
            "client_id": client_id,
          }))
        });

      let api = connect.or(root);

      let port = args.port_api;
      info!("API Port: {}", port);
      warp::serve(api)
        .run(SocketAddr::new(args.ip, port))
        .await;
      //=========================================================
    });
  }).detach();
  info!("detach!");
}

pub struct HttpServerPlugin;
impl Plugin for HttpServerPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(start);
  }
}
