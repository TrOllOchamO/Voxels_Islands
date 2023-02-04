use crate::game::world::chunk::BlockPosition;

use super::chunk::NB_BLOCKS_PER_CHUNK;
use super::pre_mesh_creator::{BlockPreMesh, ChunkPreMesh, ChunkPreMeshOneDirection};
use super::renderer::Face;
use super::resource_extractor;
use bevy::prelude::*;

use std::collections::HashMap;

const DEFAULT_COLOR: Color = Color::FUCHSIA;

pub struct FacesMesh {
    pub vertices: Vec<[f32; 3]>,
    pub vertices_normals: Vec<[f32; 3]>,
    pub vertices_uv: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl FacesMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            vertices_normals: Vec::new(),
            vertices_uv: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn add(&mut self, faces_mesh: &mut Self) {
        self.vertices.append(&mut faces_mesh.vertices);
        self.vertices_normals
            .append(&mut faces_mesh.vertices_normals);
        self.vertices_uv.append(&mut faces_mesh.vertices_uv);
        self.add_indices(&mut faces_mesh.indices);
    }

    fn add_indices(&mut self, indices: &mut Vec<u32>) {
        let old_indices_len = self.indices.len();
        self.indices.append(indices);
        let new_indices_len = self.indices.len();
        let old_vertices_len = self.vertices.len() - 4;
        for i in old_indices_len..new_indices_len {
            self.indices[i] += old_vertices_len as u32;
        }
    }
}

pub struct FacesMeshWithColor {
    pub faces_mesh: FacesMesh,
    pub color: Color,
}

impl FacesMeshWithColor {
    pub fn new() -> Self {
        Self {
            faces_mesh: FacesMesh::new(),
            color: DEFAULT_COLOR,
        }
    }
}

pub fn get_faces_mesh(chunk_pre_mesh: &ChunkPreMesh) -> HashMap<u32, FacesMeshWithColor> {
    let mut faces = HashMap::new();

    convert_chunk_pre_mesh_data_into_faces_mesh(
        &mut faces,
        &chunk_pre_mesh.positive_x,
        Face::PositiveX,
    );
    convert_chunk_pre_mesh_data_into_faces_mesh(
        &mut faces,
        &chunk_pre_mesh.negative_x,
        Face::NegativeX,
    );
    convert_chunk_pre_mesh_data_into_faces_mesh(
        &mut faces,
        &chunk_pre_mesh.positive_y,
        Face::PositiveY,
    );
    convert_chunk_pre_mesh_data_into_faces_mesh(
        &mut faces,
        &chunk_pre_mesh.negative_y,
        Face::NegativeY,
    );
    convert_chunk_pre_mesh_data_into_faces_mesh(
        &mut faces,
        &chunk_pre_mesh.positive_z,
        Face::PositiveZ,
    );
    convert_chunk_pre_mesh_data_into_faces_mesh(
        &mut faces,
        &chunk_pre_mesh.negative_z,
        Face::NegativeZ,
    );

    faces
}

fn convert_chunk_pre_mesh_data_into_faces_mesh(
    faces: &mut HashMap<u32, FacesMeshWithColor>,
    chunk_pre_mesh_in_a_direction: &ChunkPreMeshOneDirection,
    direction: Face,
) {
    for pre_mesh_inedex in 0..NB_BLOCKS_PER_CHUNK {
        let pre_mesh = chunk_pre_mesh_in_a_direction.get_pre_mesh(pre_mesh_inedex);

        if pre_mesh.get_stretch().get_as_u16() == 0 {
            continue;
        }

        let mut mesh = match direction {
            Face::PositiveX => {
                convert_block_pre_mesh_to_faces_mesh_on_positive_x(pre_mesh, pre_mesh_inedex)
            }
            Face::NegativeX => {
                convert_block_pre_mesh_to_faces_mesh_on_negative_x(pre_mesh, pre_mesh_inedex)
            }
            Face::PositiveY => {
                convert_block_pre_mesh_to_faces_mesh_on_positive_y(pre_mesh, pre_mesh_inedex)
            }
            Face::NegativeY => {
                convert_block_pre_mesh_to_faces_mesh_on_negative_y(pre_mesh, pre_mesh_inedex)
            }
            Face::PositiveZ => {
                convert_block_pre_mesh_to_faces_mesh_on_positive_z(pre_mesh, pre_mesh_inedex)
            }
            Face::NegativeZ => {
                convert_block_pre_mesh_to_faces_mesh_on_negative_z(pre_mesh, pre_mesh_inedex)
            }
        };

        let extracted_color = resource_extractor::block_color(pre_mesh.get_block().get_id());
        let color = extracted_color.unwrap_or(DEFAULT_COLOR);
        let key = color.as_rgba_u32();

        if !faces.contains_key(&key) {
            let mut faces_mesh_with_texture = FacesMeshWithColor::new();
            faces_mesh_with_texture.color = color;
            faces_mesh_with_texture.faces_mesh = mesh;
            faces.insert(key, faces_mesh_with_texture);
        } else {
            faces.get_mut(&key).unwrap().faces_mesh.add(&mut mesh);
        }
    }
}

fn convert_block_pre_mesh_to_faces_mesh_on_positive_x(
    block_pre_mesh: BlockPreMesh,
    block_index: usize,
) -> FacesMesh {
    let (_, stretch_y, stretch_z) = block_pre_mesh.get_stretch().get_final_stretch();
    let (block_x, block_y, block_z) = BlockPosition::from_block_index(block_index).pos_tuple_f32();
    let px = block_x + 1.;
    let py = block_y + 1.;
    let pz = block_z + 1.;

    let mut face = FacesMesh::new();
    face.vertices = vec![
        [px, py, pz],
        [px, py - stretch_y, pz],
        [px, py - stretch_y, pz - stretch_z],
        [px, py, pz - stretch_z],
    ];
    face.vertices_normals = vec![[1., 0., 0.], [1., 0., 0.], [1., 0., 0.], [1., 0., 0.]];
    face.vertices_uv = vec![[1., 1.], [1., 1.], [1., 1.], [1., 1.]];
    face.indices = vec![0, 1, 2, 0, 2, 3];

    face
}

fn convert_block_pre_mesh_to_faces_mesh_on_negative_x(
    block_pre_mesh: BlockPreMesh,
    block_index: usize,
) -> FacesMesh {
    let (_, stretch_y, stretch_z) = block_pre_mesh.get_stretch().get_final_stretch();
    let (block_x, block_y, block_z) = BlockPosition::from_block_index(block_index).pos_tuple_f32();
    let px = block_x;
    let py = block_y + 1.;
    let pz = block_z + 1.;

    let mut face = FacesMesh::new();
    face.vertices = vec![
        [px, py, pz],
        [px, py - stretch_y, pz],
        [px, py - stretch_y, pz - stretch_z],
        [px, py, pz - stretch_z],
    ];
    face.vertices_normals = vec![[-1., 0., 0.], [-1., 0., 0.], [-1., 0., 0.], [-1., 0., 0.]];
    face.vertices_uv = vec![[1., 1.], [1., 1.], [1., 1.], [1., 1.]];
    face.indices = vec![0, 2, 1, 0, 3, 2];

    face
}

fn convert_block_pre_mesh_to_faces_mesh_on_positive_y(
    block_pre_mesh: BlockPreMesh,
    block_index: usize,
) -> FacesMesh {
    let (stretch_x, _, stretch_z) = block_pre_mesh.get_stretch().get_final_stretch();
    let (block_x, block_y, block_z) = BlockPosition::from_block_index(block_index).pos_tuple_f32();
    let px = block_x + 1.;
    let py = block_y + 1.;
    let pz = block_z + 1.;

    let mut face = FacesMesh::new();
    face.vertices = vec![
        [px, py, pz],
        [px - stretch_x, py, pz],
        [px - stretch_x, py, pz - stretch_z],
        [px, py, pz - stretch_z],
    ];
    face.vertices_normals = vec![[0., 1., 0.], [0., 1., 0.], [0., 1., 0.], [0., 1., 0.]];
    face.vertices_uv = vec![[1., 1.], [1., 1.], [1., 1.], [1., 1.]];
    face.indices = vec![0, 2, 1, 0, 3, 2];

    face
}

fn convert_block_pre_mesh_to_faces_mesh_on_negative_y(
    block_pre_mesh: BlockPreMesh,
    block_index: usize,
) -> FacesMesh {
    let (stretch_x, _, stretch_z) = block_pre_mesh.get_stretch().get_final_stretch();
    let (block_x, block_y, block_z) = BlockPosition::from_block_index(block_index).pos_tuple_f32();
    let px = block_x + 1.;
    let py = block_y;
    let pz = block_z + 1.;

    let mut face = FacesMesh::new();
    face.vertices = vec![
        [px, py, pz],
        [px - stretch_x, py, pz],
        [px - stretch_x, py, pz - stretch_z],
        [px, py, pz - stretch_z],
    ];
    face.vertices_normals = vec![[0., -1., 0.], [0., -1., 0.], [0., -1., 0.], [0., -1., 0.]];
    face.vertices_uv = vec![[1., 1.], [1., 1.], [1., 1.], [1., 1.]];
    face.indices = vec![0, 1, 2, 0, 2, 3];

    face
}

fn convert_block_pre_mesh_to_faces_mesh_on_positive_z(
    block_pre_mesh: BlockPreMesh,
    block_index: usize,
) -> FacesMesh {
    let (stretch_x, stretch_y, _) = block_pre_mesh.get_stretch().get_final_stretch();
    let (block_x, block_y, block_z) = BlockPosition::from_block_index(block_index).pos_tuple_f32();
    let px = block_x + 1.;
    let py = block_y + 1.;
    let pz = block_z + 1.;

    let mut face = FacesMesh::new();
    face.vertices = vec![
        [px, py, pz],
        [px, py - stretch_y, pz],
        [px - stretch_x, py - stretch_y, pz],
        [px - stretch_x, py, pz],
    ];
    face.vertices_normals = vec![[0., 0., 1.], [0., 0., 1.], [0., 0., 1.], [0., 0., 1.]];
    face.vertices_uv = vec![[1., 1.], [1., 1.], [1., 1.], [1., 1.]];
    face.indices = vec![0, 2, 1, 0, 3, 2];

    face
}

fn convert_block_pre_mesh_to_faces_mesh_on_negative_z(
    block_pre_mesh: BlockPreMesh,
    block_index: usize,
) -> FacesMesh {
    let (stretch_x, stretch_y, _) = block_pre_mesh.get_stretch().get_final_stretch();
    let (block_x, block_y, block_z) = BlockPosition::from_block_index(block_index).pos_tuple_f32();
    let px = block_x + 1.;
    let py = block_y + 1.;
    let pz = block_z;

    let mut face = FacesMesh::new();
    face.vertices = vec![
        [px, py, pz],
        [px, py - stretch_y, pz],
        [px - stretch_x, py - stretch_y, pz],
        [px - stretch_x, py, pz],
    ];
    face.vertices_normals = vec![[0., 0., -1.], [0., 0., -1.], [0., 0., -1.], [0., 0., -1.]];
    face.vertices_uv = vec![[1., 1.], [1., 1.], [1., 1.], [1., 1.]];
    face.indices = vec![0, 1, 2, 0, 2, 3];

    face
}
