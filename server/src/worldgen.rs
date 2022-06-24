use shared::{
  types::{ChunkData, Block},
  consts::CHUNK_SIZE
};
use noise::{Fbm, NoiseFn};
use rand::{rngs::SmallRng, SeedableRng, Rng};

//========================================

//TODO tune these values

const TERRAIN_H_SCALE: f64       = 0.04;
const MIN_TERRAIN_HEIGHT: usize  = 100;
const TERRAIN_HEIGHT: f64        = 35.;

const GENERATE_CAVES: bool       = true;
const CAVE_TRESHOLD: f64         = 0.15; //Range: 0 - 1; Increase to *reduce* the amount of caves

const PRNG_SEED: u64             = 0x0DDB1A5E5BAD5EED; //Used only for bedrock generation

//========================================

const TERRAIN_HEIGHT_HALF: f64 = TERRAIN_HEIGHT / 2.;

pub fn generate(x: i64, y: i64) -> ChunkData {
  //Get X/Y offsets
  let x_offset: f64 = (x * CHUNK_SIZE as i64) as f64;
  let y_offset: f64 = (y * CHUNK_SIZE as i64) as f64;

  //Create ChunkData
  let mut data = ChunkData::new();
  let blocks = &mut data.0;

  //Create FBM (Fractional Brownian Motion) noise generator
  //WARNING! fbm.get() return data in range -1..=1, NOT 0..=1
  let fbm = Fbm::new();
  let cave_fbm = {
    let mut fbm = Fbm::new();
    fbm.octaves = 2;
    fbm
  };


  //Create RNG
  let mut rng = SmallRng::seed_from_u64(PRNG_SEED ^ (x as u64) ^ (y as u64));

  //Create random

  for x in 0..CHUNK_SIZE {
    for z in 0..CHUNK_SIZE {

      //Get terrain height
      let point = [x_offset + x as f64, y_offset + z as f64].map(|x| x * TERRAIN_H_SCALE);
      let h = MIN_TERRAIN_HEIGHT + (TERRAIN_HEIGHT_HALF + (fbm.get(point) * TERRAIN_HEIGHT_HALF).round()) as usize;

      //Generate terrain
      for y in 0..h {
        //Check if cave exists at the point
        let cave_check = if GENERATE_CAVES {
          let point_3d = [x_offset + x as f64, y as f64, y_offset + z as f64].map(|x| x * TERRAIN_H_SCALE);
          let point_3d_alt = [x_offset + x as f64, y as f64 + 10000., y_offset + z as f64].map(|x| x * TERRAIN_H_SCALE);
          let treshold = if y > MIN_TERRAIN_HEIGHT {
            CAVE_TRESHOLD + (1. - CAVE_TRESHOLD) * ((y - MIN_TERRAIN_HEIGHT) as f64 / TERRAIN_HEIGHT)
          } else {
            CAVE_TRESHOLD
          };
          (cave_fbm.get(point_3d).abs() > treshold) && (cave_fbm.get(point_3d_alt).abs() > treshold)
        } else { false };
        //If no cave, place a block
        if !cave_check {
          blocks[x][y][z] = Block { block_type: 1 };
        }
      }
      
      //Place "Bedrock"
      blocks[x][0][z] = Block { block_type: 1 };
      if rng.gen_bool(0.5) {
        blocks[x][1][z] = Block { block_type: 1 };
      }
    }
  }

  data
}
