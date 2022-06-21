use bevy::prelude::*;
use crate::player::MainPlayer;

fn chunk_loader(
  mut commands: Commands,
  player_transform: Query<&GlobalTransform, With<MainPlayer>>,
) {
  if let Ok(pos) = player_transform.get_single() {
    
  }
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_system(chunk_loader);
  }
}
