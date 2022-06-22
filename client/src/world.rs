use bevy::prelude::*;
use bevy::tasks::{Task, AsyncComputeTaskPool};
use crate::{
  chunk::{Chunk, ChunkPosition},
  mesh_builder::{MeshBuilder, Face}
};
use shared::{
  types::Block,
  consts::{CHUNK_HEIGHT, CHUNK_SIZE}
};
use futures_lite::future;


const MAX_STARTED_MESH_BUILD_TASKS_PER_TICK: usize = 10;


#[derive(Component, Debug)]
#[non_exhaustive]
pub enum MeshStage {
  Queued,
  Ready
}

#[derive(Component, Debug)]
pub struct MeshTask(Task<Mesh>);

fn mesh_gen_system(
  mut commands: Commands,
  chunks: Query<(Entity, &Chunk, &ChunkPosition), Without<MeshStage>>,
  pool: Res<AsyncComputeTaskPool>
) {
  for (entity, chunk, position) in chunks.iter().take(MAX_STARTED_MESH_BUILD_TASKS_PER_TICK) {
    info!("Starting mesh build task for chunk: \"{:?}\"...", position);
    let blocks = chunk.0.0.clone();
    let task = pool.spawn(async move {
      let mut builder = MeshBuilder::default();
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
      builder.build()
    });
    commands.entity(entity)
      .insert(MeshStage::Queued)
      .insert(MeshTask(task));
  }
}

fn apply_mesh_gen_tasks(
  mut commands: Commands,
  mut query: Query<(Entity, &mut MeshTask, &mut MeshStage, &ChunkPosition), With<Chunk>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for (entity, mut task, mut stage, position) in query.iter_mut() {
    if let Some(mesh) = future::block_on(future::poll_once(&mut task.0)) {
      let mut ecmd = commands.entity(entity);
      //create PbrBundle and Wireframe
      ecmd.insert_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_translation(Vec3::new(
          (position.0 * CHUNK_SIZE as i64) as f32, 
          0.0, 
          (position.1 * CHUNK_SIZE as i64) as f32
        )),
        material: materials.add(StandardMaterial {
          base_color: Color::rgb_u8(64, 128, 64),
          unlit: true,
          ..default()
        }),
        ..default()
      }).insert(bevy::pbr::wireframe::Wireframe);
      //Update MeshStage and remove MeshTask
      ecmd.remove::<MeshTask>();
      *stage = MeshStage::Ready;
      //Debug
      info!("Finish building mesh \"{:?}\"", position);
    }
  }
}

/*struct WorldMap {
  //TODO WorldMap
  map: HashMap<ChunkPosition, String>
}*/

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(
      mesh_gen_system.chain(apply_mesh_gen_tasks)
    );
  }
}
