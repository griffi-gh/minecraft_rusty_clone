use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy::utils::HashMap;
use shared::blocks::{BlockTypeManager};

#[derive(Default)]
pub struct BlockTextureAtlas(pub Option<TextureAtlas>);
impl BlockTextureAtlas {
  pub fn get(&self) -> &TextureAtlas {
    self.0.as_ref().expect("Atlas not inited")
  }
}

#[derive(Default)]
struct TextureHandles(Vec<HandleUntyped>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
  LoadingAssets,
  CreatingStuff,
  Finished
}

fn load_assets(
  mut handles: ResMut<TextureHandles>,
  server: Res<AssetServer>,
) {
  info!("Loading textures...");
  handles.0 = server.load_folder("textures").expect("Failed to load folder");
}

fn check_assets(
  mut state: ResMut<State<AppState>>,
  handles: Res<TextureHandles>,
  server: Res<AssetServer>,
) {
  if let LoadState::Loaded = server.get_group_load_state(handles.0.iter().map(|handle| handle.id)) {
    state.set(AppState::CreatingStuff).unwrap();
    info!("Finished loading textures");
  }
}

fn process_assets(
  mut atlas_res: ResMut<BlockTextureAtlas>,
  mut commands: Commands,
  mut textures: ResMut<Assets<Image>>,
  handles: Res<TextureHandles>,
) {
  let mut builder = TextureAtlasBuilder::default();
  for handle in &handles.0 {
    let handle = handle.clone_weak().typed();
    let image = textures.get(&handle).expect("Failed to load texture"); 
    builder.add_texture(handle, image);
  }
  let atlas = builder.finish(&mut textures).expect("Failed to build a texture atlas");
  info!("Inserting texture atlas");
  atlas_res.0 = Some(atlas);
  commands.remove_resource::<TextureHandles>();
  info!("Created texture atlas");
}

#[derive(Default, Clone)]
pub struct BlockTextureIndexMap(pub HashMap<String, usize>);

fn create_texture_handle_map(
  mut map: ResMut<BlockTextureIndexMap>,
  atlas: Res<BlockTextureAtlas>,
  blocks: Res<BlockTypeManager>,
  server: Res<AssetServer>,
  mut state: ResMut<State<AppState>>,
) {
  for block_type in &blocks.block_types {
    for tex in &block_type.textures {
      let partial = tex.partial();
      if map.0.contains_key(partial) {
        continue;
      }
      let full = tex.full();
      let handle: Handle<Image> = server.get_handle(&full);
      let index = *atlas.get().texture_handles.as_ref().unwrap().get(&handle).unwrap();
      map.0.insert(partial.clone(), index);
      info!("Texture \"{}\" at \"{}\" with atlas index {}", partial, &full, &index);
    }
  }
  state.set(AppState::Finished).unwrap();
}

//fn test() { info!("test") }

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<TextureHandles>();
    app.init_resource::<BlockTextureIndexMap>();
    app.init_resource::<BlockTextureAtlas>();
    app.add_state(AppState::LoadingAssets);
    app.add_system_set(SystemSet::on_enter(AppState::LoadingAssets).with_system(load_assets));
    app.add_system_set(SystemSet::on_update(AppState::LoadingAssets).with_system(check_assets));
    app.add_system_set(SystemSet::on_enter(AppState::CreatingStuff).with_system(process_assets.chain(create_texture_handle_map)));
  }
}
