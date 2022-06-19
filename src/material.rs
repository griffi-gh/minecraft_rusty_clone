use bitflags::bitflags;
use bevy::prelude::*;
use bevy::utils::HashMap;


//wtf is this

bitflags! {
  pub struct BlockMaterialFlags : u32 {
    const TRANSPARENT = 0 << 0;
    const LIQUID = 1 << 1;
  }
}

pub struct MaterialInfo {
  id: String,
  flags: u32,
  material: StandardMaterial
}

pub struct BlockMaterialManager {
  materials: Vec<MaterialInfo>,
  id_map: HashMap<String, usize>,
}
impl BlockMaterialManager {
  fn id_of<'a>(&self, mat: &'a MaterialInfo) -> &'a str {
    mat.id.as_str()
  }
  fn index_of(&self, mat: &MaterialInfo) -> Option<usize> {
    Some(*self.id_map.get(mat.id.as_str())?)
  }
  fn get_by_index(&self, index: usize) -> Option<&MaterialInfo> {
    Some(self.materials.get(index)?)
  }
  fn get_by_id(&self, id: &str) -> Option<&MaterialInfo> {
    Some(&self.materials[*self.id_map.get(id)?])
  }
  fn register(&mut self, material: MaterialInfo) {
    let id = material.id.clone();
    let index = self.materials.len();
    self.materials.push(material);
    self.id_map.insert(id, index);
  }
}
impl Default for BlockMaterialManager {
  fn default() -> Self {
    let mut new = Self {
      materials: Vec::new(),
      id_map: HashMap::new()
    };
    new.register(MaterialInfo {
      id: "default".into(),
      flags: 0,
      material: StandardMaterial {
        base_color: Color::rgb_u8(255, 255, 255),
        unlit: true,
        ..default()
      }
    });
    new
  }
}

pub struct BlockMaterialManagerPlugin;
impl BlockMaterialManagerPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.init_resource::<BlockMaterialManager>();
  }
}
