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

use bevy_renet::renet::{
  RenetConnectionConfig,
  ChannelConfig,
  ReliableChannelConfig,
  UnreliableChannelConfig,
  BlockChannelConfig
};

pub fn renet_connection_config() -> RenetConnectionConfig {
  RenetConnectionConfig {
    max_packet_size: 32 * 1024,
    channels_config: vec![
      ChannelConfig::Reliable(ReliableChannelConfig::default()),
      ChannelConfig::Unreliable(UnreliableChannelConfig::default()),
      ChannelConfig::Block(BlockChannelConfig {
          packet_budget: 30000,
          message_send_queue_size: 64,
          ..Default::default()
      }),
    ],
    ..Default::default()
  }
}
