use serde::{Deserialize, Serialize};
use crate::types::{
  CompressedChunkData,
  ChatMessage
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerMessages {
  ChatMessage { message: ChatMessage },
  PlayerConnected {id: u64 },
  PlayerDisconnected { id: u64 },
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ServerBlockChannelMessages {
  ChunkData {
    data: CompressedChunkData,
    position: (i64, i64)
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientMessages {
  ChatMessage { message: String },
  ChunkRequest { x: i64, y: i64 },
}
