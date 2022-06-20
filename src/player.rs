use bevy::prelude::*;

pub struct FirstPersonController;
impl FirstPersonController {
  fn startup(
    mut commands: Commands
  ) {
    commands.spawn_bundle(PerspectiveCameraBundle {
      transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
    });
  }
}
impl Plugin for FirstPersonController {
  fn build(&self, app: &mut App) {
    app.add_startup_system(Self::startup);
  }
}
