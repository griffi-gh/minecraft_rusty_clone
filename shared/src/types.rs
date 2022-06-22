use crate::consts::{CHUNK_HEIGHT, CHUNK_SIZE};
use serde::{Serialize, Deserialize};
use serde_with::serde_as;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Block {
  block_type: u16
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
