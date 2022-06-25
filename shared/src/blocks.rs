//TODO
use bevy::{prelude::*, utils::HashMap};
use crate::types::CubeFace;

pub struct TexturePath(String);
impl TexturePath {
  pub fn partial(&self) -> &String {
    &self.0
  }
  pub fn full(&self) -> String {
    //TODO sanitize path
    format!("textures/{}.png", &self.0)
  }
}
impl From<String> for TexturePath {
  fn from(string: String) -> Self {
    Self(string)
  }    
}
impl From<&str> for TexturePath {
  fn from(string: &str) -> Self {
    Self(string.into())
  }    
}

const INVALID_KEY: &str = "__invalid_key__";

const fn side_textures(map: [(CubeFace, usize); 6]) -> [usize; 6] {
  let mut index = 0;
  let mut array = [0; 6];
  let mut seen = [false; 6];
  while index < map.len() {
    let key = map[index].0 as usize;
    assert!(!seen[key], "Duplicate side key in side_textures()");
    seen[key] = true;
    array[key] = map[index].1;
    index += 1;
  }
  array
}
macro_rules! single_texture {
  ($name: expr) => { vec![$name.into()] };
}

#[repr(u16)]
pub enum BlockFlags {
  FlagAir    = 1 << 0,
  FlagSolid  = 1 << 1,
  FlagLiquid = 1 << 2,
}

pub struct BlockMetadata {
  pub index: Option<usize>,
  pub key: String,
  pub name: String,
  pub textures: Vec<TexturePath>,
  pub side_textures: [usize; 6],
  pub optimize_sides: [bool; 6],
  pub flags: u16,
}
impl Default for BlockMetadata {
  fn default() -> Self {
    Self {
      index: None,
      key: INVALID_KEY.into(),
      name: "block".into(),
      textures: Vec::new(),
      side_textures: [0; 6],
      optimize_sides: [true; 6],
      flags: BlockFlags::FlagSolid as u16,
    }
  }
}
impl BlockMetadata {
  pub fn is_air(&self) -> bool {
    return (self.flags & BlockFlags::FlagAir as u16) > 0;
  }
  pub fn is_solid(&self) -> bool {
    return (self.flags & BlockFlags::FlagSolid as u16) > 0;
  }
  pub fn is_liquid(&self) -> bool {
    return (self.flags & BlockFlags::FlagLiquid as u16) > 0;
  }
}

#[derive(Default)]
pub struct BlockTypeManager {
  block_types: Vec<BlockMetadata>,
  block_map: HashMap<String, usize>,
}
impl BlockTypeManager {
  //TODO rename this function
  pub fn register_vec(&mut self, blocks: Vec<BlockMetadata>) {
    for block in blocks {
      self.register(block);
    }
  }

  //TODO proper error handling instead of panics
  pub fn register(&mut self, mut block: BlockMetadata) {
    assert!(&block.key[..] != INVALID_KEY && block.key.len() > 0, "Invalid or empty block key");
    assert!(!self.block_map.contains_key(&block.key), "Block with key \"{}\" is already registered", block.key);
    if block.textures.len() > 0 {
      for index in block.side_textures {
        assert!(index < block.textures.len(), "Invalid texture index in side_textures");
      }
    }
    let index = self.block_types.len();
    block.index = Some(index);
    self.block_map.insert(block.key.clone(), index);
    self.block_types.push(block);
  }

  pub fn get_by_index(&self, index: usize) -> Option<&BlockMetadata> {
    Some(self.block_types.get(index)?)
  }
  pub fn get_by_key(&self, key: &str) -> Option<&BlockMetadata> {
    Some(&self.block_types[*self.block_map.get(key)?])
  }
}

fn register_blocks(
  mut blocks: ResMut<BlockTypeManager>
) {
  blocks.register_vec(vec![
    BlockMetadata {
      key: "air".into(),
      flags: BlockFlags::FlagAir as u16,
      ..default()
    },
    BlockMetadata {
      key: "dirt".into(),
      name: "Dirt Block".into(),
      textures: single_texture!("dirt"),
      ..default()
    },
    BlockMetadata {
      key: "grass".into(),
      name: "Grass Block".into(),
      textures: vec![
        "grass_block_top".into(),
        "grass_block_side".into(),
        "dirt".into()
      ],
      side_textures: side_textures([
        (CubeFace::Top   , 0),
        (CubeFace::Front , 1),
        (CubeFace::Left  , 1),
        (CubeFace::Right , 1),
        (CubeFace::Back  , 1),
        (CubeFace::Bottom, 2),
      ]),
      ..default()
    },
    BlockMetadata {
      key: "stone".into(),
      name: "Stone Block".into(),
      textures: single_texture!("stone"),
      ..default()
    },
    BlockMetadata {
      key: "bedrock".into(),
      name: "Bedrock".into(),
      textures: single_texture!("bedrock"),
      ..default()
    }
  ]);
}

pub struct BlockManagerPlugin;
impl Plugin for BlockManagerPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<BlockTypeManager>();
    app.add_startup_system(register_blocks);
  }
}
