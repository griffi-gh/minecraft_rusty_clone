use shared::{
  blocks::BlockTypeManager,
  types::{chunk::ChunkData, block::Block},
  consts::{CHUNK_SIZE, CHUNK_HEIGHT}
};
use noise::{Fbm, NoiseFn};
use rand::{rngs::SmallRng, SeedableRng, Rng};

//========================================
const TERRAIN_NOISE_SCALE: f64  = 0.04;
const TERRAIN_OCTAVES: usize    = 6;
const MIN_TERRAIN_HEIGHT: usize = 100;
const TERRAIN_HEIGHT: f64       = 35.;
const TERRAIN_STONE_START: f64  = 0.10;//Range: 0 - 1

const GENERATE_CAVES: bool      = true;
const CAVE_NOISE_SCALE: f64     = 0.04;
const CAVE_OCTAVES: usize       = 2;
const CAVE_TRESHOLD: f64        = 0.15; //Range: 0 - 1; Increase to *reduce* the amount of caves

const MAX_BEDROCK_HEIGHT: usize = 3;

const PRNG_SEED: u64            = 0x0DDB1A5E5BAD5EED;

const ORE_NOISE_SCALE: f64      = 0.133;
const ORE_OCTAVES: usize        = 1;
const DIAMOND_ORE_AMOUNT: f64   = 0.05;
const COAL_ORE_AMOUNT: f64      = 0.2;
const IRON_ORE_AMOUNT: f64      = 0.1;
const GOLD_ORE_AMOUNT: f64      = 0.075;
const EMERALD_ORE_AMOUNT: f64   = 0.025;

//========================================

const TERRAIN_HEIGHT_HALF: f64 = TERRAIN_HEIGHT / 2.;

pub fn generate(x: i64, y: i64, blocks: &BlockTypeManager) -> ChunkData {
  let index_of = |key| blocks.get_by_key(key).unwrap().index.unwrap() as u16;

  let air_index     = index_of("air");
  let dirt_index    = index_of("dirt");
  let grass_index   = index_of("grass");
  let stone_index   = index_of("stone");
  let bedrock_index = index_of("bedrock");

  let diamond_ore_index = index_of("diamond_ore");
  let coal_ore_index    = index_of("coal_ore");
  let emerald_ore_index = index_of("emerald_ore");
  let gold_ore_index    = index_of("gold_ore");
  let iron_ore_index    = index_of("iron_ore");

  let ore_amounts = [
    (coal_ore_index, COAL_ORE_AMOUNT),
    (iron_ore_index, IRON_ORE_AMOUNT),
    (gold_ore_index, GOLD_ORE_AMOUNT),
    (diamond_ore_index, DIAMOND_ORE_AMOUNT),
    (emerald_ore_index, EMERALD_ORE_AMOUNT),
  ];

  //Get X/Y offsets
  let x_offset: f64 = (x * CHUNK_SIZE as i64) as f64;
  let y_offset: f64 = (y * CHUNK_SIZE as i64) as f64;

  //Create ChunkData
  let mut data = ChunkData::new();
  let blocks = &mut data.0;

  //Create FBM (Fractional Brownian Motion) noise generator
  //fbm.get() return data in range -1..=1

  let mut terrain_fbm = Fbm::new();
  terrain_fbm.octaves = TERRAIN_OCTAVES;

  let mut cave_fbm = Fbm::new();
  cave_fbm.octaves = CAVE_OCTAVES;

  let mut ore_fbm = Fbm::new();
  ore_fbm.octaves = ORE_OCTAVES;

  //Create RNG
  let mut rng = SmallRng::seed_from_u64(PRNG_SEED ^ (x as u64) ^ (y as u64));

  for x in 0..CHUNK_SIZE {
    for z in 0..CHUNK_SIZE {
      //Fill with air
      for y in 0..CHUNK_HEIGHT {
        blocks[x][y][z] = Block { block_type: air_index };
      }

      //Get terrain height
      let point = [x_offset + x as f64, y_offset + z as f64].map(|x| x * TERRAIN_NOISE_SCALE);
      let h = MIN_TERRAIN_HEIGHT + (TERRAIN_HEIGHT_HALF + (terrain_fbm.get(point) * TERRAIN_HEIGHT_HALF).round()) as usize;

      //Generate terrain
      for y in 0..h {
        let stone_probability = {
          if y > MIN_TERRAIN_HEIGHT { 
            (1. - ((y - MIN_TERRAIN_HEIGHT) as f64 / (TERRAIN_HEIGHT * TERRAIN_STONE_START))).min(1.).max(0.)
          } else { 1. }
        };
        blocks[x][y][z] = Block { 
          block_type: if rng.gen_bool(stone_probability) { 
            stone_index 
          } else if y == (h - 1) { 
            grass_index 
          } else {
            dirt_index
          }
        }; 
      }

      //Generate caves
      if GENERATE_CAVES {
        for y in 0..h {
          let point_3d = [x_offset + x as f64, y as f64, y_offset + z as f64].map(|x| x * CAVE_NOISE_SCALE);
          let point_3d_alt = [x_offset + x as f64, y as f64 + 10000., y_offset + z as f64].map(|x| x * CAVE_NOISE_SCALE);
          let treshold = if y > MIN_TERRAIN_HEIGHT {
            CAVE_TRESHOLD + (1. - CAVE_TRESHOLD) * ((y - MIN_TERRAIN_HEIGHT) as f64 / TERRAIN_HEIGHT)
          } else {
            CAVE_TRESHOLD
          };
          let is_cave = (cave_fbm.get(point_3d).abs() > treshold) && (cave_fbm.get(point_3d_alt).abs() > treshold);
          if is_cave {
            blocks[x][y][z] = Block { block_type: air_index };
          }
        }
      }

      //Generate ores
      for y in 0..h {
        if blocks[x][y][z].block_type != stone_index {
          continue;
        }
        for (i, (ore_index, amount)) in ore_amounts.iter().enumerate() {
          let point_3d = [x_offset + x as f64, (CHUNK_HEIGHT * i) as f64 + y as f64, y_offset + z as f64].map(|x| x * ORE_NOISE_SCALE);
          let val = ore_fbm.get(point_3d);
          if ((val + 1.) / 2.) > (1. - *amount) {
            blocks[x][y][z] = Block { block_type: *ore_index };
          }
        }
      }

      //Add Bedrock
      {
        let mut probability = 1.;
        for y in 0..MAX_BEDROCK_HEIGHT {
          if rng.gen_bool(probability) {
            blocks[x][y][z] = Block { block_type: bedrock_index };
          }
          probability /= 2.;
        }
      }
    }
  }

  data
}
