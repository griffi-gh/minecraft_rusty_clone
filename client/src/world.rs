use bevy::prelude::*;

fn mesh_gen_system(
  mut commands: Commands
) {
  
}

struct WorldPlugin;
impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(mesh_gen_system);
  }
}
