use bevy::prelude::*;
use bevy_eventwork::{
  EventworkPlugin,
  tcp::{NetworkSettings, TcpProvider}
};
use shared::networking::register_messages_client;

mod chunk;
mod world;
mod networking;

fn main() {
  let mut app = App::new();

  app.insert_resource(WindowDescriptor {
    title: "Minecraft clone".into(),
    ..default()
  });
  app.add_plugins(DefaultPlugins);
  
  app.insert_resource(bevy::tasks::TaskPoolBuilder::new().build());

  app.insert_resource(NetworkSettings::default());
  app.add_plugin(EventworkPlugin::<
    TcpProvider,
    bevy::tasks::TaskPool,
  >::default());
  register_messages_client(&mut app);

  app.add_startup_system(networking::connect);
  app.add_system(networking::handle_network_events);

  app.run();
}
