use bevy::prelude::*;
use bevy_flycam::{NoCameraPlayerPlugin, FlyCam};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainPlayer;

pub struct FirstPersonControllerPlugin;
impl FirstPersonControllerPlugin {
  fn startup(
    mut commands: Commands
  ) {
    commands.spawn_bundle(PerspectiveCameraBundle {
      transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
    }).insert(FlyCam).insert(Player).insert(MainPlayer);
  }
}
impl Plugin for FirstPersonControllerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(Self::startup)
      .add_plugin(NoCameraPlayerPlugin);
  }
}
