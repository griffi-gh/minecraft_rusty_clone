use bevy::prelude::*;
use bevy::tasks::TaskPoolBuilder;
use bevy_egui::EguiPlugin;

use shared::blocks::BlockManagerPlugin;

mod mesh_builder;
mod chunk;
mod world;
mod networking;
mod assets;
mod player;
mod chat;

use networking::NetworkingPlugin;
use world::WorldPlugin;
use assets::AssetLoaderPlugin;
use player::PlayerPlugin;
use chat::ChatPlugin;

fn main() {
  let mut app = App::new();

  app.insert_resource(WindowDescriptor {
    title: "Minecraft clone".into(),
    ..default()
  });
  app.insert_resource(Msaa { samples: 4 });

  app.add_plugins(DefaultPlugins);
  app.insert_resource(TaskPoolBuilder::new().build());

  //app.add_plugin(bevy::pbr::wireframe::WireframePlugin);
  app.add_plugin(EguiPlugin);

  app.insert_resource(ClearColor(Color::rgb_u8(135, 206, 235)));
  app.insert_resource(AmbientLight {
    color: Color::WHITE,
    brightness: 1.0,
  });

  app.add_plugin(BlockManagerPlugin);
  app.add_plugin(PlayerPlugin);
  app.add_plugin(AssetLoaderPlugin);
  app.add_plugin(NetworkingPlugin);
  app.add_plugin(WorldPlugin);
  app.add_plugin(ChatPlugin);
  
  app.run();
}
