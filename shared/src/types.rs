use crate::consts::{CHUNK_HEIGHT, CHUNK_SIZE};
use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use compress::rle;
use std::io::{Write, Read};

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
    let mut decoder_buffer = Vec::new();
    let compressed_data = self.0.to_owned();
    rle::Decoder::new(&compressed_data[..])
      .read_to_end(&mut decoder_buffer).unwrap();
    bincode::deserialize(&decoder_buffer[..]).unwrap()
  }
}
