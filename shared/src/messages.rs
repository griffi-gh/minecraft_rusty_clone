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
    max_packet_size: 128 * 1024,
    channels_config: vec![
      ChannelConfig::Reliable(ReliableChannelConfig::default()),
      ChannelConfig::Unreliable(UnreliableChannelConfig {
        max_message_size: u64::MAX,
        packet_budget: u64::MAX,
        ..Default::default()
      }),
      ChannelConfig::Block(BlockChannelConfig::default()),
    ],
    ..Default::default()
  }
}
