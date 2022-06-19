use bevy::prelude::*;

pub struct WorldManager;
impl WorldManager {
  fn setup(
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
          base_color: Color::rgb(0.8, 0.7, 0.6),
          unlit: true,
          ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
  }  
}

impl Plugin for WorldManager {
  fn build(&self, app: &mut App) {
    app.add_startup_system(Self::setup);
  }
}
