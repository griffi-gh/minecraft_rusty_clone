use bevy::prelude::*;
use iyes_loopless::prelude::*;

use bevy::{
  tasks::{Task, AsyncComputeTaskPool},
  utils::HashSet,
};
use shared::blocks::BlockShape;
use crate::{
  GameState,
  networking::RequestChunk,
  player::{ChunkLocation, MainPlayer},
  assets::{AssetLoaderState, BlockTextureAtlas, BlockTextureIndexMap},
  
  mesh_builder::MeshBuilder
};
use shared::{
  types::{
    CubeFace,
    block::Block, 
    chunk::{ChunkDataComponent, ChunkPosition, Chunk},
  },
  consts::{CHUNK_HEIGHT, CHUNK_SIZE, DEFAULT_CLIENT_VIEW_DIST},
  blocks::{BlockTypeManager, BlockMetadata}
};
use futures_lite::future;
use std::ops::RangeInclusive;

const MAX_STARTED_MESH_BUILD_TASKS_PER_TICK: usize = 10;
const MAX_PROCESSED_FINISHED_BUILD_TASKS_PER_TICK: usize = usize::MAX;

#[derive(Component, Debug)]
#[non_exhaustive]
pub enum MeshStage {
  Queued,
  Ready
}

#[derive(Component, Debug)]
pub struct MeshTask(Task<Mesh>);

fn chunk_distance(pos: &ChunkPosition, loc: &ChunkLocation) -> usize {
  ((pos.0 - loc.0).abs()).max((pos.1 - loc.1).abs()) as _
}

fn update_loaded_chunks_around_player (
  mut commands: Commands,
  mut ev_reqest: EventWriter<RequestChunk>,
  chunks: Query<(Entity, &ChunkPosition), With<Chunk>>,
  player_chunk: Query<&ChunkLocation, (With<MainPlayer>, Changed<ChunkLocation>)>
) {
  if player_chunk.is_empty() { 
    return;
  }
  let player_chunk = player_chunk.single();

  info!("Player moved to chunk {:?}", &player_chunk);

  //Unload chunks and build HashSet of chunks that are still loaded
  let mut loaded = HashSet::new();
  for (entity, chunk_pos) in chunks.iter() {
    if chunk_distance(chunk_pos, player_chunk) <= DEFAULT_CLIENT_VIEW_DIST {
      loaded.insert(*chunk_pos);
    } else {
      commands.entity(entity).despawn();
      info!("Unloaded {:?}", chunk_pos);
    }
  }

  //Load new chunks
  const RANGE: RangeInclusive<i64> = (-(DEFAULT_CLIENT_VIEW_DIST as i64))..=(DEFAULT_CLIENT_VIEW_DIST as i64);
  for x in RANGE {
    for y in RANGE {
      let position = ChunkPosition(x + player_chunk.0, y + player_chunk.1);
      if !loaded.contains(&position) {
        info!("Requesting chunk {:?}", &position);
        ev_reqest.send(position.into());
      }
    }
  }
}

fn mesh_gen_system(
  mut commands: Commands,
  chunks: Query<(Entity, &ChunkDataComponent, &ChunkPosition), Without<MeshStage>>,
  pool: Res<AsyncComputeTaskPool>,
  ref atlas: Res<BlockTextureAtlas>,
  block_types: Res<BlockTypeManager>,
  index_map: Res<BlockTextureIndexMap>
) {
  for (entity, chunk, position) in chunks.iter().take(MAX_STARTED_MESH_BUILD_TASKS_PER_TICK) {
    info!("Starting mesh build task for chunk: \"{:?}\"...", position);

    let blocks = chunk.0.0.clone();
    let textures = atlas.get().textures.clone();
    let atlas_size = atlas.get().size;
    let metadatas: Vec<BlockMetadata> = block_types.block_types.clone();
    let tex_map = index_map.0.clone();

    let task = pool.spawn(async move {
      let mut builder = MeshBuilder::default();
      for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
          for z in 0..CHUNK_SIZE {
            //=========================

            let check_block = |x: &Block| {
              let meta = &metadatas[x.block_type as usize];
              meta.is_air() || meta.shape != BlockShape::Cube
            };

            let block: Block = blocks[x][y][z];
            if check_block(&block) { continue; }

            //=========================

            let coord = [x as u8, y as u8, z as u8];
            let query = |dx: i8, dy: i8, dz: i8| -> bool {
              let (qx, qy, qz) = (
                x as isize + dx as isize, 
                y as isize + dy as isize, 
                z as isize + dz as isize 
              );
              const MAX_H: isize = (CHUNK_SIZE - 1) as isize;
              const MAX_V: isize = (CHUNK_HEIGHT - 1) as isize;
              if qx < 0 || qy < 0 || qz < 0 || qx > MAX_H || qy > MAX_V || qz > MAX_H {
                true
              } else {
                check_block(&blocks[qx as usize][qy as usize][qz as usize])
              }
            };
            /*const UV: [[f32; 2]; 4] = [
              [1.0, 1.0],
              [1.0, 0.0],
              [0.0, 1.0],
              [0.0, 0.0],
            ];*/

            let face_uv = |face: CubeFace| {
              //what
              //the
              //fuck
              let meta = &metadatas[block.block_type as usize];
              let tex_index = meta.face_textures[face as usize];
              let tex_path = meta.textures[tex_index].partial();
              let atlas_tex_idx = *tex_map.get(tex_path).expect("No texture");
              let min = textures[atlas_tex_idx].min / atlas_size;
              let max = textures[atlas_tex_idx].max / atlas_size;
              [
                [max.x, max.y],
                [max.x, min.y],
                [min.x, max.y],
                [min.x, min.y],
              ]
            };
            
            match &metadatas[block.block_type as usize].shape {
              BlockShape::Cube => {
                builder.add_face_if(query(0, 1,0), CubeFace::Top,    coord, face_uv(CubeFace::Top));
                builder.add_face_if(query(0,0,-1), CubeFace::Front,  coord, face_uv(CubeFace::Front));
                builder.add_face_if(query(-1,0,0), CubeFace::Left,   coord, face_uv(CubeFace::Left));
                builder.add_face_if(query(1, 0,0), CubeFace::Right,  coord, face_uv(CubeFace::Right));
                builder.add_face_if(query(0, 0,1), CubeFace::Back,   coord, face_uv(CubeFace::Back));
                builder.add_face_if(query(0,-1,0), CubeFace::Bottom, coord, face_uv(CubeFace::Bottom));
              },
              _ => {
                error!("UNIMPLEMENTED SHAPE");
                panic!("UNIMPLEMENTED SHAPE");
              }
            }
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
  ref atlas: Res<BlockTextureAtlas>,
) {
  for (entity, mut task, mut stage, position) in query.iter_mut().take(MAX_PROCESSED_FINISHED_BUILD_TASKS_PER_TICK) {
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
          base_color: Color::WHITE,
          base_color_texture: Some(atlas.0.as_ref().unwrap().texture.as_weak()),

          reflectance: 0.,
          metallic: 0.,
          perceptual_roughness: 0.5,

          //unlit: true,
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

fn despawn_world (
  mut commands: Commands,
  chunks: Query<Entity, With<Chunk>>
) {
  for chunk in chunks.iter() {
    commands.entity(chunk).despawn();
  }
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app.add_system_set(
      ConditionSet::new()
        .label("WorldMain")
        .run_in_bevy_state(AssetLoaderState::Finished)
        .run_in_state(GameState::InGame)
        .with_system(mesh_gen_system)
        .with_system(apply_mesh_gen_tasks)
        .into()
    );
    app.add_system_set(
      ConditionSet::new()
        .label("AfterWorldMain")
        .after("WorldMain")
        .run_in_bevy_state(AssetLoaderState::Finished)
        .run_in_state(GameState::InGame)
        .with_system(update_loaded_chunks_around_player)
        .into()
    );
    app.add_exit_system(
      GameState::InGame, 
      despawn_world
    );
  }
}
