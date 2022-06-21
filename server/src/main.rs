use bevy::prelude::*;

fn main() {
  let mut app = App::new();

  app.add_plugins(MinimalPlugins);
  app.add_plugin(bevy::log::LogPlugin);

  app.run();
}
