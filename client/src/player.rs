use bevy::{prelude::*};
use bevy_flycam::PlayerPlugin as FlyCamPlugin;
use shared::consts::CHUNK_SIZE;

#[derive(Component, Default)]
pub struct MainPlayer;

#[derive(Component, Default)]
pub struct NetPlayer;

#[derive(Component, Default)]
pub struct Player;

//TODO ChunkLocation: Rename, the name is too similar to ChunkPosition
//TODO ChunkLocation: Move (maybe to shared?) from player.rs since it works with all Entities
#[derive(Component, Default, Debug)]
pub struct ChunkLocation(pub i64, pub i64);

fn update_chunk_location(
  mut players: Query<(&mut ChunkLocation, &GlobalTransform), (With<Transform>, Changed<Transform>)>
) {
  for mut player in players.iter_mut() {
    let pos = player.1.translation;
    let new_pos = (
      (pos.x / (CHUNK_SIZE as f32)).floor() as i64,
      (pos.z / (CHUNK_SIZE as f32)).floor() as i64
    );
    if player.0.0 != new_pos.0 || player.0.1 != new_pos.1 {
      player.0.0 = new_pos.0;
      player.0.1 = new_pos.1;
    }
  }
}

fn setup(
  mut commands: Commands, 
  fly: Query<Entity, (With<bevy_flycam::FlyCam>, Without<Player>)>
) {
  if fly.is_empty() { return; }
  commands.entity(fly.single())
    .insert(Player)
    .insert(MainPlayer)
    .insert(ChunkLocation::default());
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugin(FlyCamPlugin);
    app.add_system(setup); //TODO convert to startup system
    app.add_system(update_chunk_location);
  }
}
