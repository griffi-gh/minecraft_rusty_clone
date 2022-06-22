use bevy::prelude::*;
use bevy::tasks::TaskPoolBuilder;

mod chunk;
mod world;
mod networking;

use networking::NetworkingPlugin;

fn main() {
  let mut app = App::new();

  app.insert_resource(WindowDescriptor {
    title: "Minecraft clone".into(),
    ..default()
  });
  app.add_plugins(DefaultPlugins);
  app.insert_resource(TaskPoolBuilder::new().build());

  app.add_plugin(NetworkingPlugin);

  app.run();
}
