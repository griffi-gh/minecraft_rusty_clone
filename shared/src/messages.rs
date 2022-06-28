use serde::{Deserialize, Serialize};
use crate::types::{
  CompressedChunkData,
  ChatMessage
};

#[derive(Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub enum ServerMessages {
  ChatMessage { message: ChatMessage },
  PlayerConnected {id: u64 },
  PlayerDisconnected { id: u64 },
  ChunkData {
    data: CompressedChunkData,
    position: (i64, i64)
  },
}

#[derive(Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub enum ClientMessages {
  ChatMessage { message: String },
  ChunkRequest { x: i64, y: i64 },
}
