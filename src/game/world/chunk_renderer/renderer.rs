use super::chunk::Chunk;
use super::pre_mesh_creator::{ChunkPreMesh, NeighborChunks};
use super::pre_mesh_to_bundle_conveter::get_faces_mesh;
use crate::game::world::chunk::ChunkData;
use crate::game::world::world::CHUNK_SIZE_I32;
use crate::world::World;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::render::mesh::{self, PrimitiveTopology};
use std::collections::HashMap;

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
    for (parent_chunk, mesh_entity) in chunks_meshes.iter() {
        if parent_chunk.get() == chunk_entity {
            commands.entity(mesh_entity).despawn_recursive();
        }
    }
}

fn get_chunk_data(
    chunk_coords: (i32, i32, i32),
    world_chunks: &HashMap<(i32, i32, i32), Entity>,
    chunks: &Query<(Entity, &Parent, &Chunk)>,
) -> Option<ChunkData> {
    if !world_chunks.contains_key(&chunk_coords) {
        return None;
    }

    match world_chunks.get(&chunk_coords) {
        None => None,
        Some(chunk_entity) => {
            let (_, _, chunk) = chunks.get(*chunk_entity).unwrap();
            Some(chunk.blocks.clone())
        }
    }
}

fn get_neighbor_chunks(
    chunk_coords: (i32, i32, i32),
    world_chunks: &HashMap<(i32, i32, i32), Entity>,
    chunks: &Query<(Entity, &Parent, &Chunk)>,
) -> NeighborChunks {
    let (x, y, z) = chunk_coords;
    NeighborChunks {
        positive_x: get_chunk_data((x + CHUNK_SIZE_I32, y, z), world_chunks, chunks),
        negative_x: get_chunk_data((x - CHUNK_SIZE_I32, y, z), world_chunks, chunks),
        positive_z: get_chunk_data((x, y, z + CHUNK_SIZE_I32), world_chunks, chunks),
        negative_z: get_chunk_data((x, y, z - CHUNK_SIZE_I32), world_chunks, chunks),
    }
}

pub fn generate_chunk_mesh_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut worlds: Query<(Entity, &mut World)>,
    chunks: Query<(Entity, &Parent, &Chunk)>,
    mut chunks_meshes: Query<(&Parent, Entity), With<ChunkMeshTag>>,
) {
    for (world_entity, mut world_struct) in worlds.iter_mut() {
        let world_chunks = get_world_chunks(world_entity, &chunks);
        let mut chunks_to_render = get_chunks_to_render(&world_struct, &world_chunks, &chunks);
        for (chunk_coords, chunk_to_render_entity) in chunks_to_render.drain() {
            remove_old_meshes(chunk_to_render_entity, &mut chunks_meshes, &mut commands);
            let (_, _, chunk) = chunks.get(chunk_to_render_entity).unwrap();
            let neighbor_chunks = get_neighbor_chunks(chunk_coords, &world_chunks, &chunks);

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
                world
                    .entity_mut(chunk_to_render_entity)
                    .push_children(&entities);
            });
        }
    }
}

fn get_chunks_to_render(
    world: &World,
    world_chunks: &HashMap<(i32, i32, i32), Entity>,
    chunks: &Query<(Entity, &Parent, &Chunk)>,
) -> HashMap<(i32, i32, i32), Entity> {
    let mut chunks_to_render = world_chunks.clone();
    chunks_to_render.retain(|_, chunk_entity| {
        let (_, _, chunk) = chunks.get(*chunk_entity).unwrap();
        let chunk_coords = chunk.get_coords().to_tuple();
        world.chunks_to_render.contains(&chunk_coords)
    });
    chunks_to_render
}

fn get_world_chunks(
    world_entity: Entity,
    chunks: &Query<(Entity, &Parent, &Chunk)>,
) -> HashMap<(i32, i32, i32), Entity> {
    let mut world_chunks = HashMap::new();
    for (chunk_entity, chunk_parent, chunk) in chunks.iter() {
        if world_entity == chunk_parent.get() {
            let chunk_coords = chunk.get_coords().to_tuple();
            world_chunks.insert(chunk_coords, chunk_entity);
        }
    }
    world_chunks
}
