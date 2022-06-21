use bevy::prelude::*;

mod chunk;
mod world;

fn main() {
  App::new()
    .insert_resource(WindowDescriptor {
      title: "Minecraft clone".into(),
      ..default()
    })
    .add_plugins(DefaultPlugins)
    .run();
}
