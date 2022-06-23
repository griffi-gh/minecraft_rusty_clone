use bevy::prelude::*;
use bevy_eventwork::{AppNetworkMessage, NetworkMessage, tcp::TcpProvider};
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
impl NetworkMessage for ChunkDataRequestMessage {
  const NAME: &'static str = "ReqChunk";
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChunkDataMessage {
  pub data: CompressedChunkData,
  pub x: i64,
  pub y: i64
}
impl NetworkMessage for ChunkDataMessage {
  const NAME: &'static str = "ChunkData";
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewUserChatMessageMessage(pub UserChatMessage);
impl NetworkMessage for NewUserChatMessageMessage {
  const NAME: &'static str = "UsrChatMsg";
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewChatMessageMessage(pub ChatMessage);
impl NetworkMessage for NewChatMessageMessage {
  const NAME: &'static str = "ChatMsg";
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthMessage(pub AuthData);
impl NetworkMessage for AuthMessage {
  const NAME: &'static str = "AuthMessage";
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthResultMessage(pub AuthResult);
impl NetworkMessage for AuthResultMessage {
  const NAME: &'static str = "AuthResult";
}

/// The client registers messages that arrives from the server, so that
/// it is prepared to handle them. Otherwise, an error occurs.
pub fn register_messages_client(app: &mut App) {
  app.listen_for_message::<AuthResultMessage, TcpProvider>();
  app.listen_for_message::<ChunkDataMessage, TcpProvider>();
  app.listen_for_message::<NewChatMessageMessage, TcpProvider>();
}

/// The server registers messages that arrives from a client, so that
/// it is prepared to handle them. Otherwise, an error occurs.
pub fn register_messages_server(app: &mut App) {
  app.listen_for_message::<AuthMessage, TcpProvider>();
  app.listen_for_message::<ChunkDataRequestMessage, TcpProvider>();
  app.listen_for_message::<NewUserChatMessageMessage, TcpProvider>();
}
