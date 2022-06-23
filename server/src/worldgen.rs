use shared::{
  types::{ChunkData, Block},
  consts::CHUNK_SIZE
};
use noise::{Fbm, NoiseFn};

const TERRAIN_H_SCALE:    f64   = 0.05;
const MIN_TERRAIN_HEIGHT: usize = 10;
const MAX_TERRAIN_HEIGHT: f64   = 40.;

const TERRAIN_HEIGHT: f64 = MAX_TERRAIN_HEIGHT - MIN_TERRAIN_HEIGHT as f64;
const TERRAIN_HEIGHT_HALF: f64 = TERRAIN_HEIGHT / 2.;

pub fn generate(x: i64, y: i64) -> ChunkData {
  //Get X/Y offsets
  let x_offset: f64 = (x * CHUNK_SIZE as i64) as f64;
  let y_offset: f64 = (y * CHUNK_SIZE as i64) as f64;

  //Create ChunkData
  let mut data = ChunkData::new();
  let blocks = &mut data.0;

  //Create FBM (Fractional Brownian Motion) noise generator
  let fbm = {
    let mut fbm = Fbm::new();
    fbm.octaves = 3;
    fbm
  };

  for x in 0..CHUNK_SIZE {
    for z in 0..CHUNK_SIZE {
      let point = [x_offset + x as f64, y_offset + z as f64].map(|x| x * TERRAIN_H_SCALE);
      let h = MIN_TERRAIN_HEIGHT + (TERRAIN_HEIGHT_HALF + (fbm.get(point) * TERRAIN_HEIGHT_HALF).round()) as usize - MIN_TERRAIN_HEIGHT;
      for y in 0..h {
        blocks[x][y][z] = Block{ block_type: 1 };
      }
    }
  }

  data
}
