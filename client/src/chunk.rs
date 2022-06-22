use bevy::prelude::*;
use shared::{
  consts::{CHUNK_SIZE, CHUNK_HEIGHT},
  types::{Block, ChunkData}
};

#[derive(Component, Clone)]
pub struct Chunk(pub ChunkData);

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkPosition(pub i64, pub i64);
