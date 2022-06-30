use crate::consts::{CHUNK_HEIGHT, CHUNK_SIZE};
use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use std::time::SystemTime;

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum CubeFace {
  Top    = 0,
  Front  = 1,
  Left   = 2,
  Right  = 3,
  Back   = 4,
  Bottom = 5,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
  pub block_type: u16
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct ChunkData (
  #[serde_as(as = "Box<[[[_; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]>")]
  pub Box<[[[Block; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]>
);
impl ChunkData {
  pub fn new() -> Self {
    Self (
      Box::new([[[Block{block_type: 0}; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE])
    )
  }
  pub fn new_test() -> Self {
    Self ({
      let mut new = Box::new([[[Block{block_type: 0}; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]);
      for x in 0..CHUNK_SIZE {
        for y in 0..40 {
          for z in 0..CHUNK_SIZE {
            new[x][y][z] = Block{block_type: 1};
          }
        }
      }
      new
    })
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompressedChunkData(pub Vec<u8>);
impl From<&ChunkData> for CompressedChunkData {
  fn from(chunk_data: &ChunkData) -> Self {
    let data = bincode::serialize(&chunk_data).unwrap();
    let cumpressed = compress_prepend_size(&data[..]);
    Self(cumpressed)
  }
}
impl Into<ChunkData> for &CompressedChunkData {
  fn into(self) -> ChunkData {
    let decumpressed = decompress_size_prepended(&self.0[..]).unwrap();
    bincode::deserialize(&decumpressed[..]).unwrap()
  }
}
impl From<ChunkData> for CompressedChunkData {
  fn from(chunk_data: ChunkData) -> Self {
    (&chunk_data).into()
  }
}
impl Into<ChunkData> for CompressedChunkData {
  fn into(self) -> ChunkData {
    (&self).into()
  }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
  pub message: String,
  pub from: String,
  pub timestamp: SystemTime,
  pub is_system: bool,
}
impl ChatMessage {
  pub fn new(message: String, from: String) -> Self {
    ChatMessage {
      message, from,
      timestamp: SystemTime::now(),
      is_system: false
    }
  }
  pub fn system_message(message: String) -> Self {
    ChatMessage {
      message,
      from: "[SYSTEM]".into(),
      timestamp: SystemTime::now(),
      is_system: true
    }
  }
}
