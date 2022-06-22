use bevy::prelude::*;
use bevy_eventwork::{AppNetworkMessage, NetworkMessage, tcp::TcpProvider};
use serde::{Deserialize, Serialize};
use compress::rle;
use std::io::{Write, Read};
use crate::types::ChunkData;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ChunkDataRequestMessage {
  pub x: i64,
  pub y: i64
}
impl NetworkMessage for ChunkDataRequestMessage {
  const NAME: &'static str = "ReqChunk";
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompressedChunkData(pub Vec<u8>);
impl From<ChunkData> for CompressedChunkData {
  fn from(chunk_data: ChunkData) -> Self {
    let data = bincode::serialize(&chunk_data).unwrap();
    let mut encoder = rle::Encoder::new(Vec::new());
    encoder.write_all(&data[..]).unwrap();
    drop(data);
    Self(encoder.finish().0)
  }
}
impl Into<ChunkData> for CompressedChunkData {
  fn into(self) -> ChunkData {
    let mut decoder_buf = Vec::new();
    rle::Decoder::new(&self.0[..]).read_to_end(&mut decoder_buf).unwrap();
    bincode::deserialize(&decoder_buf[..]).unwrap()
  }
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

/// The client registers messages that arrives from the server, so that
/// it is prepared to handle them. Otherwise, an error occurs.
pub fn register_messages_client(app: &mut App) {
  app.listen_for_message::<ChunkDataMessage, TcpProvider>();
}

/// The server registers messages that arrives from a client, so that
/// it is prepared to handle them. Otherwise, an error occurs.
pub fn register_messages_server(app: &mut App) {
  app.listen_for_message::<ChunkDataRequestMessage, TcpProvider>();
}
