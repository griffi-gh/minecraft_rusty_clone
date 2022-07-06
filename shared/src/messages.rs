use serde::{Deserialize, Serialize};
use bevy::prelude::Vec3;
use crate::types::{
  chunk::CompressedChunkData,
  chat::ChatMessage,
  player::PlayerInitData
};

#[derive(Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub enum ServerToClientMessages {
  ChatMessage { message: ChatMessage },
  PlayerConnected {
    id: u64,
    init_data: PlayerInitData
  },
  PlayerDisconnected { id: u64 },
  InitData {
    self_init: PlayerInitData,
    player_init: Vec<(u64, PlayerInitData)>,
    chat_messages: Vec<ChatMessage>,
  },
  PlayerSync {
    id: u64,
    new_pos: Vec3
  },
  ChunkData {
    data: CompressedChunkData,
    position: (i64, i64)
  },
}

#[derive(Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub enum ClientToServerMessages {
  PlayerMove { new_pos: Vec3 },
  ChatMessage { message: String },
  ChunkRequest { x: i64, y: i64 },
}
