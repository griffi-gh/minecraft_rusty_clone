use bevy::prelude::*;
use bevy::tasks::TaskPoolBuilder;

mod mesh_builder;
mod chunk;
mod world;
mod networking;

use networking::NetworkingPlugin;
use world::WorldPlugin;

fn main() {
  let mut app = App::new();

  app.insert_resource(WindowDescriptor {
    title: "Minecraft clone".into(),
    ..default()
  });
  app.add_plugins(DefaultPlugins);
  app.insert_resource(TaskPoolBuilder::new().build());

  app.add_plugin(NetworkingPlugin);
  app.add_plugin(WorldPlugin);
  
  app.run();
}
