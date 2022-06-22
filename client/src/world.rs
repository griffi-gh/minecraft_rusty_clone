use bevy::prelude::*;
use crate::{
  chunk::{Chunk, ChunkPosition},
  mesh_builder::{MeshBuilder, Face}
};
use shared::{
  types::Block,
  consts::{CHUNK_HEIGHT, CHUNK_SIZE}
};

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct HasMesh;

fn mesh_gen_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  chunks: Query<(Entity, &Chunk, &ChunkPosition), Without<HasMesh>>
) {
  for (entity, chunk, position) in chunks.iter() {
    info!("Building chunk mesh for chunk: \"{:?}\"...", position);
    let mut builder = MeshBuilder::default();
    let blocks = &chunk.0.0;
    for x in 0..CHUNK_SIZE {
      for y in 0..CHUNK_HEIGHT {
        for z in 0..CHUNK_SIZE {
          let block = blocks[x][y][z];
          if block.block_type == 0 {
            continue;
          }
          //=========================
          let coord = [x as u8, y as u8, z as u8];
          let query = |dx: i8, dy: i8, dz: i8| -> bool {
            let (qx, qy, qz) = (
              x as isize + dx as isize, 
              y as isize + dy as isize, 
              z as isize + dz as isize 
            );
            let check_block = |x: &Block| x.block_type == 0;
            const MAX_H: isize = (CHUNK_SIZE - 1) as isize;
            const MAX_V: isize = (CHUNK_HEIGHT - 1) as isize;
            if qx < 0 || qy < 0 || qz < 0 || qx > MAX_H || qy > MAX_V || qz > MAX_H {
              true
            } else {
              check_block(&blocks[qx as usize][qy as usize][qz as usize])
            }
          };
          const UV: [[f32; 2]; 4] = [
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 0.0],
            [1.0, 1.0]
          ];
          builder.add_face_if(query(0, 1,0), Face::Top,    coord, UV);
          builder.add_face_if(query(0,0,-1), Face::Front,  coord, UV);
          builder.add_face_if(query(-1,0,0), Face::Left,   coord, UV);
          builder.add_face_if(query(1, 0,0), Face::Right,  coord, UV);
          builder.add_face_if(query(0, 0,1), Face::Back,   coord, UV);
          builder.add_face_if(query(0,-1,0), Face::Bottom, coord, UV);
          //=========================
        }
      }
    }
    commands.entity(entity)
      .insert(HasMesh)
      .insert_bundle(PbrBundle {
        mesh: meshes.add(builder.build()),
        material: materials.add(StandardMaterial {
          base_color: Color::rgb_u8(128, 255, 128),
          unlit: true,
          ..default()
        }),
        ..default()
      });
  }
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(mesh_gen_system);
  }
}
