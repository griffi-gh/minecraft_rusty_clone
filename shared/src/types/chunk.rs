use bevy::prelude::*;
use bevy::utils::HashMap;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use super::block::Block;
use crate::consts::{CHUNK_SIZE, CHUNK_HEIGHT};

#[derive(Component, Clone, Copy)]
pub struct Chunk;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPosition(pub i64, pub i64);
impl ChunkPosition {
  pub fn new(x: i64, y: i64) -> Self { Self(x, y) }
  pub fn x(&self) -> i64 { self.0 }
  pub fn y(&self) -> i64 { self.1 }
  pub fn xy(&self) -> (i64, i64) { (self.0, self.1) }
}
#[derive(Component, Clone)] 
pub struct ChunkDataComponent(pub ChunkData);

#[derive(Default, Clone)] 
pub struct ChunkMap(HashMap<(i64, i64), Entity>);
impl ChunkMap {
  pub fn get(&self, pos: ChunkPosition) -> Option<Entity> {
    self.0.get(&(pos.0, pos.1)).map(|x| *x)
  }
  pub fn insert(&mut self, pos: ChunkPosition, entity: Entity) {
    self.0.insert((pos.0, pos.1), entity);
  }
  pub fn remove(&mut self, pos: ChunkPosition) {
    self.0.remove(&(pos.0, pos.1));
  }
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct ChunkData (
  #[serde_as(as = "Box<[[[_; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]>")]
  pub Box<[[[Block; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]>
);
impl ChunkData {
  pub fn new() -> Self {
    Self (Box::new([[[Block{block_type: 0}; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]))
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
