use bevy::prelude::*;
use bevy::{
  tasks::AsyncComputeTaskPool,
  utils::HashMap
};
use bevy_renet::renet::ConnectToken;

use tokio::runtime::Runtime as TokioRuntime;
use warp::Filter;

use base64;

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
      //TODO server metadata
      let connect = 
        warp::path!("connect")
        .and(warp::query::<HashMap<String, String>>())
        .map(move |_query: HashMap<String, String>| {
          //TODO Password auth
          //TODO Rate limiting
          info!("Connect token requested");
          let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
          let client_id = current_time.as_millis() as u64;
          let server_addresses = vec![SocketAddr::new(args.ip, args.port)];
          let mut buffer = Vec::new();
          ConnectToken::generate(
            current_time, PROTOCOL_ID, EXPIRE_SECONDS, client_id, TIMEOUT_SECONDS, 
            server_addresses, None, &private_key
          ).unwrap().write(&mut buffer).unwrap();
          base64::encode(&buffer)
        });

      let api = connect;

      let port = args.port + 1;
      info!("Starting API on port {}", port);
      warp::serve(api)
        .run(SocketAddr::new(args.ip, port))
        .await;
      //=========================================================
    });
  }).detach();
}

pub struct HttpServerPlugin;
impl Plugin for HttpServerPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(start);
  }
}
