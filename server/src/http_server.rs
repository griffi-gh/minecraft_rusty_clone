use bevy::prelude::*;
use bevy::{
  tasks::AsyncComputeTaskPool,
  utils::HashMap
};
use bevy_renet::renet::{ConnectToken, NETCODE_USER_DATA_BYTES};

use tokio::runtime::Runtime as TokioRuntime;
use warp::{Filter, http::status::StatusCode};

use base64;

use bincode;
use serde_json::json;

use std::{
  net::SocketAddr, time::SystemTime,
};
use crate::{
  Args, server::PrivateKey
};
use shared::{
  consts::PROTOCOL_ID,
  types::AuthUserData,
  utils::check_username,
};

const EXPIRE_SECONDS: u64 = 300;
const TIMEOUT_SECONDS: i32 = 15;

type ConnectReply = warp::reply::WithStatus<warp::reply::Json>;
fn connect_reply_ok(token: String, port: u16, client_id: u64) -> ConnectReply {
  warp::reply::with_status(
    warp::reply::json(&json!({
      "success": true,
      "code": 200,
      "token": token,
      "port": port,
      "client_id": client_id,
    })),
    StatusCode::OK
  )
} 
fn connect_reply_error(error: Option<&'static str>) -> ConnectReply {
  warp::reply::with_status(
    warp::reply::json(&json!({
      "success": false,
      "code": 500,
      "reason": format!(
        "Internal server error {}{}", 
        if error.is_some() { ": " } else { "" },
        error.unwrap_or("")
      )
    })),
    StatusCode::INTERNAL_SERVER_ERROR
  )
} 
fn connect_reply_validation_fail(error: &'static str) -> ConnectReply {
  warp::reply::with_status(
    warp::reply::json(&json!({
      "success": false,
      "code": 422,
      "reason": format!("Validation error: {}", error)
    })),
    StatusCode::UNPROCESSABLE_ENTITY
  )
} 


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
        .map(move |query: HashMap<String, String>| {
          //TODO Password auth
          //TODO Rate limiting
          info!("Connect token requested");

          //Verify data
          let status = {
            let name_option = query.get("username");
            match name_option {
              Some(name) => match check_username(name) {
                true => Ok((name,)),
                false => Err("Invalid username")
              }
              None => Err("Missing username")
            }
          };

          match status {
            Err(error) => connect_reply_validation_fail(error),
            Ok((username,)) => {
              //Create user data
              let user_data = {
                let user_data = bincode::serialize(&AuthUserData {
                  username: username.clone()
                }).unwrap();

                if (user_data.len() > u8::MAX as usize) || (user_data.len() >= NETCODE_USER_DATA_BYTES) {
                  error!("Userdata too long");
                  return connect_reply_error(Some("Userdata is too long"));
                }
                
                let mut user_data_buf = [0u8; NETCODE_USER_DATA_BYTES];
                user_data_buf[0] = user_data.len() as u8;
                for i in 0..user_data.len() {
                  user_data_buf[i + 1] = user_data[i];
                }

                user_data_buf
              };

              //Create token
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
                server_addresses.clone(), Some(&user_data), &private_key
              ).expect("Failed to generate the token").write(&mut buffer).unwrap();

              connect_reply_ok(
                base64::encode(&buffer),
                server_addresses[0].port(),
                client_id
              )
            }
          }
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
