use shared::{
  types::{ChunkData, Block},
  consts::CHUNK_SIZE
};
use noise::{Fbm, NoiseFn};
use rand::{rngs::SmallRng, SeedableRng, Rng};

//========================================

//TODO tune these values

const TERRAIN_H_SCALE:    f64    = 0.04;
const MIN_TERRAIN_HEIGHT: usize  = 85;
const TERRAIN_HEIGHT: f64        = 35.;

const GENERATE_CAVES: bool       = true;
//macaroni caves
const CAVE_GAP_WIDTH: f64        = 0.125; //Range: 0 - 1
const TOP_CAVE_GAP_REDUCE: f64   = 0.8;   //Range: 0 - 1 (percentage of CAVE_GAP_WIDTH)
//beeg ones (e.g. old cave system, they are still used)
const CAVE_AMOUNT: f64           = 0.4;   //Range: 0 - 1 
const CAVE_TOP_REDUCE: f64       = 0.15;  //Range: 0 - 1

const HANGING_CHECK: bool        = false; //Can look pretty weird, so disabled by default
const HANGING_TRESHOLD: u8       = 6;

const PRNG_SEED: u64             = 0x0DDB1A5E5BAD5EED; //Used only for bedrock generation

//========================================

const TERRAIN_MAX_HEIGHT: f64 = MIN_TERRAIN_HEIGHT as f64 + TERRAIN_HEIGHT;
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

        //Generate caves
        let cave_check = if GENERATE_CAVES {
          let point_3d = [x_offset + x as f64, y as f64, y_offset + z as f64].map(|x| x * TERRAIN_H_SCALE);
          
          let beeg_caves = ((fbm.get(point_3d) + 1.) / 2.) < (CAVE_AMOUNT - (y as f64 / TERRAIN_MAX_HEIGHT).min(1.) * CAVE_TOP_REDUCE);

          let pasta_caves = {
            let gap_width = CAVE_GAP_WIDTH - CAVE_GAP_WIDTH * TOP_CAVE_GAP_REDUCE * (y as f64 / TERRAIN_MAX_HEIGHT).min(1.);
            ((0.5-(gap_width/2.))..=(0.5+(gap_width/2.))).contains(&((cave_fbm.get(point_3d) + 1.) / 2.))
          };

          beeg_caves || pasta_caves
        } else { false };

        //If no cave, place a block
        if !cave_check {
          blocks[x][y][z] = Block { block_type: 1 };
        }
      }

      //"Collapse" gaps
      //Disabled by default!
      if HANGING_CHECK {
        let mut gap: u8 = 0;
        let mut hanging = false;
        for y in MIN_TERRAIN_HEIGHT..h {
          if !hanging {
            if blocks[x][y][z].block_type == 0 {
              gap += 1;
              continue;
            } else {
              gap = 0;
              hanging = gap > HANGING_TRESHOLD;
            }
          }
          //Check again
          if hanging {
            blocks[x][y][z] = Block { block_type: 0 };
          }
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
