use bevy::prelude::*;
use bevy::asset::LoadState;

pub struct BlockTextureAtlas(pub TextureAtlas);

#[derive(Default)]
struct TextureHandles(Vec<HandleUntyped>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
  LoadingAssets,
  Finished,
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
  if let LoadState::Loaded = server.get_group_load_state(handles.0.iter().map(|handle| handle.id))
  {
    state.set(AppState::Finished).unwrap();
    info!("Finished loading textures");
  }
}

fn process_assets(
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
  commands.insert_resource(
    BlockTextureAtlas(atlas)
  );
  commands.remove_resource::<TextureHandles>();
  info!("Created texture atlas, ready to build chunks");
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<TextureHandles>();
    app.add_state(AppState::LoadingAssets);
    app.add_system_set(SystemSet::on_enter(AppState::LoadingAssets).with_system(load_assets));
    app.add_system_set(SystemSet::on_update(AppState::LoadingAssets).with_system(check_assets));
    app.add_system_set(SystemSet::on_enter(AppState::Finished).with_system(process_assets));
  }
}