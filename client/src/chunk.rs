use bevy::prelude::*;
use shared::{
  consts::{CHUNK_SIZE, CHUNK_HEIGHT},
  types::{Block, ChunkData}
};

#[derive(Component)]
struct Chunk(ChunkData);

#[derive(Component)]
struct ChunkPosition(i64, i64);
