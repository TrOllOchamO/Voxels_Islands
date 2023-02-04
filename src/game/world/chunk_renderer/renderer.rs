use super::chunk::Chunk;
use super::pre_mesh_creator::{ChunkPreMesh, NeighborChunks};
use super::pre_mesh_to_bundle_conveter::get_faces_mesh;
use crate::world::World;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::render::mesh::{self, PrimitiveTopology};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Face {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}

#[derive(Component, Default)]
pub struct ChunkMeshTag;

#[derive(Bundle, Default)]
struct ChunkMeshBundle {
    tag: ChunkMeshTag,
    pbr_bundle: PbrBundle,
}

fn remove_old_meshes(
    chunk_entity: Entity,
    chunks_meshes: &mut Query<(&Parent, Entity), With<ChunkMeshTag>>,
    commands: &mut Commands,
) {
    for (parent_chunk, mesh_entity) in chunks_meshes.iter_mut() {
        if parent_chunk.get() == chunk_entity {
            commands.entity(mesh_entity).despawn_recursive();
        }
    }
}

pub fn generate_chunk_mesh_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut worlds: Query<&mut World>,
    mut chunks: Query<(Entity, &Chunk), With<Chunk>>,
    mut chunks_meshes: Query<(&Parent, Entity), With<ChunkMeshTag>>,
) {
    for mut world_struct in worlds.iter_mut() {
        for (chunk_entity, chunk) in chunks.iter_mut() {
            if commands.get_entity(chunk_entity).is_none() {
                continue;
            }

            let chunk_coords = chunk.get_coords().to_tuple();
            if !world_struct.chunks_to_render.contains(&chunk_coords) {
                continue;
            }

            remove_old_meshes(chunk_entity, &mut chunks_meshes, &mut commands);

            let neighbor_chunks = NeighborChunks {
                positive_x: None,
                negative_x: None,
                positive_z: None,
                negative_z: None,
            };

            let mut chunk_pre_meshes = ChunkPreMesh::new(chunk.as_slice());
            chunk_pre_meshes.optimise(&neighbor_chunks);
            let mut chunk_faces = get_faces_mesh(&chunk_pre_meshes);

            let mut bundles = Vec::new();
            for (_, faces_mesh_with_color) in chunk_faces.drain() {
                let indices = faces_mesh_with_color.faces_mesh.indices;
                let positions = faces_mesh_with_color.faces_mesh.vertices;
                let normals = faces_mesh_with_color.faces_mesh.vertices_normals;
                let uvs = faces_mesh_with_color.faces_mesh.vertices_uv;

                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                mesh.set_indices(Some(mesh::Indices::U32(indices)));
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

                let pbr_bundle = PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(faces_mesh_with_color.color.into()),
                    ..default()
                };

                let final_bundle = ChunkMeshBundle {
                    pbr_bundle,
                    ..Default::default()
                };

                bundles.push(final_bundle);
            }

            world_struct.chunks_to_render.remove(&chunk_coords);
            commands.add(move |world: &mut bevy::ecs::prelude::World| {
                let entities: Vec<_> = world.spawn_batch(bundles).collect();
                world.entity_mut(chunk_entity).push_children(&entities);
            });
        }
    }
}
