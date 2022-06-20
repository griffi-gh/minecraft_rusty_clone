use bevy::prelude::*;
mod world;
mod player;

fn test_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // plane
  commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
      material: materials.add(StandardMaterial {
        base_color: Color::rgb(0.3, 0.5, 0.3),
        unlit: true,
        ..default()
      }),
      ..default()
  });
  // cube
  commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(StandardMaterial {
        base_color: Color::rgb(0.6, 0.2, 0.2),
        unlit: true,
        ..default()
      }),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..default()
  });
}  

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(test_scene)
    .run();
}
