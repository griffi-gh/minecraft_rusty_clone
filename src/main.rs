use bevy::prelude::*;
mod world;
mod material;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(world::WorldManager)
    .run();
}
