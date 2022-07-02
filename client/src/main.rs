use bevy::prelude::*;
use bevy::tasks::TaskPoolBuilder;
use bevy_egui::EguiPlugin;

use iyes_loopless::prelude::AppLooplessStateExt;
use shared::blocks::BlockManagerPlugin;

pub(crate) mod mesh_builder;
pub(crate) mod chunk;
pub(crate) mod world;
pub(crate) mod networking;
pub(crate) mod assets;
pub(crate) mod player;
pub(crate) mod chat;

use networking::NetworkingPlugin;
use world::WorldPlugin;
use assets::AssetLoaderPlugin;
use player::PlayerPlugin;
use chat::ChatPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum GameState {
  MainMenu,
  Connecting,
  InGame
}

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

  app.add_loopless_state(GameState::MainMenu);

  app.add_plugin(BlockManagerPlugin);
  app.add_plugin(PlayerPlugin);
  app.add_plugin(AssetLoaderPlugin);
  app.add_plugin(NetworkingPlugin);
  app.add_plugin(WorldPlugin);
  app.add_plugin(ChatPlugin);
  
  app.run();
}
