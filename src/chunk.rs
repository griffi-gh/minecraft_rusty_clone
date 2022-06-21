use bevy::prelude::*;
use crate::mesh_builder::{MeshBuilder, Face};

pub const CHUNK_SIZE_H: usize = 16;
pub const CHUNK_SIZE_V: usize = 256;

const CHUNK_MAX_H: usize = CHUNK_SIZE_H - 1;
const CHUNK_MAX_V: usize = CHUNK_SIZE_V - 1;

const UV: [[f32; 2]; 4] = [
  [0.0, 0.0],
  [0.0, 1.0],
  [1.0, 0.0],
  [1.0, 1.0]
];

#[derive(Component)]
pub struct Chunk {
  blocks: [[[u32; CHUNK_SIZE_H]; CHUNK_SIZE_V]; CHUNK_SIZE_H],
  x: isize, y: isize
}

impl Chunk {
  pub fn new(x: isize, y: isize) -> Self {
    Self {
      blocks: [[[0; CHUNK_SIZE_H]; CHUNK_SIZE_V]; CHUNK_SIZE_H],
      x, y
    }
  }

  pub fn test(mut self) -> Self {
    for x in 0..CHUNK_SIZE_H {
      for y in 0..40_usize {
        for z in 0..CHUNK_SIZE_H {
          self.blocks[x][y][z] = 1;
        }
      }
    }
    self
  }

  pub fn build_mesh(&self, surrounding_chunks: Option<[&Chunk; 4]>) -> Mesh {
    //surrounding_chunks: 
    //  0 
    //1   2 → +x
    //  3 
    //  ↓
    //  +z
    const NEG_Z: usize = 0;
    const NEG_X: usize = 1;
    const POS_X: usize = 2;
    const POS_Z: usize = 3;

    let mut builder = MeshBuilder::default();
    for x in 0..CHUNK_SIZE_H {
      for y in 0..CHUNK_SIZE_V {
        for z in 0..CHUNK_SIZE_H {
          let block = &self.blocks[x][y][z];
          if *block == 0 {
            continue;
          }
          let coord = [x as u8, y as u8, z as u8];
          let query = |dx: i8, dy: i8, dz: i8| -> bool {
            let check_block = |x| x == 0;
            let (qx, qy, qz) = (
              x as isize + dx as isize, 
              y as isize + dy as isize, 
              z as isize + dz as isize 
            );
            let chunk_max_h: isize = CHUNK_MAX_H.try_into().unwrap();
            if qx < 0 || qy < 0 || qz < 0 || qx > chunk_max_h || qy > CHUNK_MAX_V.try_into().unwrap() || qz > chunk_max_h {
              if let Some(surrounding_chunks) = surrounding_chunks {
                if qx < 0 {
                  check_block(surrounding_chunks[NEG_X].blocks[CHUNK_MAX_H][y][z])
                } else if qx > chunk_max_h {
                  check_block(surrounding_chunks[POS_X].blocks[0][y][z])
                } else if qz < 0 {
                  check_block(surrounding_chunks[NEG_Z].blocks[x][y][CHUNK_MAX_H])
                } else if qz > chunk_max_h {
                  check_block(surrounding_chunks[POS_Z].blocks[x][y][0])
                } else { true }
              } else { true }
            } else {
              check_block(self.blocks[qx as usize][qy as usize][qz as usize])
            }
          };
          builder.add_face_if(query(0, 1,0), Face::Top,    coord, UV);
          builder.add_face_if(query(0,0,-1), Face::Front,  coord, UV);
          builder.add_face_if(query(-1,0,0), Face::Left,   coord, UV);
          builder.add_face_if(query(1, 0,0), Face::Right,  coord, UV);
          builder.add_face_if(query(0, 0,1), Face::Back,   coord, UV);
          builder.add_face_if(query(0,-1,0), Face::Bottom, coord, UV);
        }
      }
    }
    builder.build()
  }
}
