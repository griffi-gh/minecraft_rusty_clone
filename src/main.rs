use bevy::prelude::*;

pub(crate) mod mesh_builder;
mod chunk;
mod player;

use player::FirstPersonController;

fn test_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(chunk::Chunk::new(0, 0).test().build_mesh(None)),
    material: materials.add(StandardMaterial {
      base_color: Color::rgb(0.6, 0.2, 0.2),
      unlit: true,
      ..default()
    }),
    transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.05,0.05,0.05)),
    ..default()
  });
}  

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(WindowDescriptor {
      title: "Minecraft clone".into(),
      ..default()
    })
    .add_system(bevy::input::system::exit_on_esc_system)
    .add_startup_system(test_scene)
    .add_plugin(FirstPersonController)
    .run();
}
