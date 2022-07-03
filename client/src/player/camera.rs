//90% stolen from `bevy_flycam` crate

use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Component)] 
pub struct Camera {
  pub sensitivity: f32,
  pub speed: f32,
}
impl Default for Camera {
  fn default() -> Self {
    Self {
      sensitivity: 0.00012,
      speed: 12.,
    }
  }
}

#[derive(Component)] 
pub struct LockedOn;

fn cursor_locked(
  windows: ResMut<Windows>
) -> bool {
  windows.get_primary().unwrap().cursor_locked()
}

fn camera_move(
  keys: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut query: Query<(&mut Transform, &Camera), With<LockedOn>>,
) {
  if query.is_empty() { return }
  let (mut transform, options) = query.single_mut();

  let local_z = transform.local_z();
  let forward_vec = -Vec3::new(local_z.x, 0., local_z.z);
  let right_vec = Vec3::new(local_z.z, 0., -local_z.x);

  let mut velocity = Vec3::ZERO;
  for key in keys.get_pressed() {
    match key {
      KeyCode::W => velocity += forward_vec,
      KeyCode::S => velocity -= forward_vec,
      KeyCode::A => velocity -= right_vec,
      KeyCode::D => velocity += right_vec,
      KeyCode::Space => velocity += Vec3::Y,
      KeyCode::LShift => velocity -= Vec3::Y,
      _ => {}
    }
  }
  velocity = velocity.normalize_or_zero();

  if velocity.length() > 0. {
    transform.translation += velocity * time.delta_seconds() * options.speed
  }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_system_set(
      ConditionSet::new()
        .run_if(cursor_locked)
        .with_system(camera_move)
        .into()
    );
  }
}
