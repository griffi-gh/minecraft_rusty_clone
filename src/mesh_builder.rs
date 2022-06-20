use bevy::prelude::*;
use bevy::render::mesh::{PrimitiveTopology, Indices};

#[derive(Default)]
pub struct MeshBuilder {
  vertices: Vec<[f32; 3]>,
  normals: Vec<[f32; 3]>,
  uvs: Vec<[f32; 2]>,
  indices: Vec<u32>,
  faces: u32
}

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Face {
  Top    = 0,
  Front  = 1,
  Left   = 2,
  Right  = 3,
  Back   = 4,
  Bottom = 5,
}

const FACE_VERTICES: [[[f32; 3]; 4]; 6] = [
  [[0., 1., 0.], [0., 1., 1.], [1., 1., 0.], [1., 1., 1.]],
  [[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [1., 1., 0.]],
  [[0., 0., 1.], [0., 1., 1.], [0., 0., 0.], [0., 1., 0.]],
  [[1., 0., 0.], [1., 1., 0.], [1., 0., 1.], [1., 1., 1.]],
  [[1., 0., 1.], [1., 1., 1.], [0., 0., 1.], [0., 1., 1.]],
  [[0., 0., 1.], [0., 0., 0.], [1., 0., 1.], [1., 0., 0.]]
];
pub const FACE_NORMALS: [[f32; 3]; 6] = [
  [0., 1., 0.],
  [0., 0., -1.],
  [-1., 0., 0.],
  [1., 0., 0.],
  [0., 0., 1.],
  [0., -1., 0.]
];
pub const TRIANGLES: [u32; 6] = [0, 1, 2, 2, 1, 3];

impl MeshBuilder {
  pub fn add_face(&mut self, face: Face, coord: [u8; 3], uvs: [[f32; 2]; 4]) {
    //Get face index from Face
    let face_index = face as usize;
    
    //Vertices
    self.vertices.extend_from_slice(
      &FACE_VERTICES[face_index].map(|mut vert| {
        vert[0] += coord[0] as f32;
        vert[1] += coord[1] as f32;
        vert[2] += coord[2] as f32;
        vert
      })
    );

    //Indices
    self.indices.extend_from_slice(
      &TRIANGLES.map(|x| {
        x + (4 * self.faces)
      })
    );

    //Normals
    self.normals.extend(
      std::iter::repeat(FACE_NORMALS[face_index]).take(4)
    );

    //UVs
    self.uvs.extend_from_slice(&uvs);

    //Increment face counter
    self.faces += 1;
  }

  pub fn add_face_if(&mut self, condition: bool, face: Face, coord: [u8; 3], uvs: [[f32; 2]; 4]) {
    if condition {
      self.add_face(face, coord, uvs)
    }
  }

  pub fn build(self) -> Mesh{
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
    mesh.set_indices(Some(Indices::U32(self.indices)));
    mesh
  }
}
