use bevy::prelude::*;

pub(crate) mod mesh_builder;
pub(crate) mod world;
pub(crate) mod chunk;
pub(crate) mod player;

use player::FirstPersonController;

fn test_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for cx in 0..3_usize {
    for cy in 0..3_usize {
      commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(chunk::Chunk::new(cx as isize, cy as isize).test().build_mesh(None)),
        material: materials.add(StandardMaterial {
          base_color: Color::rgb(0.6, 0.2, 0.2),
          unlit: true,
          ..default()
        }),
        transform: Transform::from_xyz((cx*chunk::CHUNK_SIZE_H) as f32, 0.0, (cy*chunk::CHUNK_SIZE_H) as f32),
        ..default()
      }).insert(bevy::pbr::wireframe::Wireframe);
    }
  }
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
    .add_plugin(bevy::pbr::wireframe::WireframePlugin)
    .run();
}
