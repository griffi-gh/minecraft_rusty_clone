use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::types::{
  CompressedChunkData,
  UserChatMessage,
  ChatMessage, 
  AuthData,
  AuthResult
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ChunkDataRequestMessage {
  pub x: i64,
  pub y: i64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChunkDataMessage {
  pub data: CompressedChunkData,
  pub x: i64,
  pub y: i64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewUserChatMessageMessage(pub UserChatMessage);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewChatMessageMessage(pub ChatMessage);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthMessage(pub AuthData);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthResultMessage(pub AuthResult);

// /// The client registers messages that arrives from the server, so that
// /// it is prepared to handle them. Otherwise, an error occurs.
// pub fn register_messages_client(app: &mut App) {
//   app.listen_for_message::<AuthResultMessage, TcpProvider>();
//   app.listen_for_message::<ChunkDataMessage, TcpProvider>();
//   app.listen_for_message::<NewChatMessageMessage, TcpProvider>();
// }

// /// The server registers messages that arrives from a client, so that
// /// it is prepared to handle them. Otherwise, an error occurs.
// pub fn register_messages_server(app: &mut App) {
//   app.listen_for_message::<AuthMessage, TcpProvider>();
//   app.listen_for_message::<ChunkDataRequestMessage, TcpProvider>();
//   app.listen_for_message::<NewUserChatMessageMessage, TcpProvider>();
// }
