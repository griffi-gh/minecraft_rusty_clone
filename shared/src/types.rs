use crate::consts::{CHUNK_HEIGHT, CHUNK_SIZE};
use serde::{Serialize, Deserialize};
use serde_with::serde_as;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
  block_type: u16
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ChunkData (
  #[serde_as(as = "Box<[[[_; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]>")]
  Box<[[[Block; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]>
);
