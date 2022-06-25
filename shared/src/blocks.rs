//TODO
use bevy::{prelude::*, utils::HashMap};

pub struct BlockMetadata {
  pub key: String,
  pub name: String,
  pub textures: Vec<String>,
  //pub optimize_sides: [bool; 6]
}

pub struct BlockTypeManager {
  block_types: Vec<BlockMetadata>,
  block_map: HashMap<String, BlockMetadata>,
}
impl BlockTypeManager {
  pub fn register_vec(&mut self, blocks: Vec<BlockMetadata>) {
    for block in blocks {
      self.register(block);
    }
  }
  pub fn register(&mut self, block: BlockMetadata) {
    todo!();
  }
  pub fn get_by_key(&self, key: String) {
    todo!();
  }
}

fn register_default_blocks(
  mut blocks: ResMut<BlockTypeManager>
) {
  blocks.register_vec(vec![
    BlockMetadata {
      key: "dirt".into(),
      name: "Dirt Block".into(),
      textures: vec!["dirt".into()]
    },
    BlockMetadata {
      key: "stone".into(),
      name: "Stone Block".into(),
      textures: vec!["stone".into()]
    },
    BlockMetadata {
      key: "stone".into(),
      name: "Stone Block".into(),
      textures: vec!["stone".into()]
    }
  ]);
}
