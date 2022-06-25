use bevy::prelude::*;
use bevy::tasks::TaskPoolBuilder;

use shared::blocks::BlockManagerPlugin;

mod mesh_builder;
mod chunk;
mod world;
mod networking;
mod assets;

use networking::NetworkingPlugin;
use world::WorldPlugin;
use assets::AssetLoaderPlugin;

use bevy_flycam::PlayerPlugin as FlyCamPlugin;

fn main() {
  let mut app = App::new();

  app.insert_resource(WindowDescriptor {
    title: "Minecraft clone".into(),
    ..default()
  });

  app.add_plugins(DefaultPlugins);
  app.insert_resource(TaskPoolBuilder::new().build());

  app.add_plugin(bevy::pbr::wireframe::WireframePlugin);

  app.add_plugin(FlyCamPlugin);

  app.add_plugin(BlockManagerPlugin);

  app.add_plugin(AssetLoaderPlugin);
  app.add_plugin(NetworkingPlugin);
  app.add_plugin(WorldPlugin);

  app.run();
}
