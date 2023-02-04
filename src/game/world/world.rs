use super::chunk_renderer::renderer::generate_chunk_mesh_system;
use super::chunks_manager::{handle_generated_chunks_system, manage_chunks_system};
use super::world_generator::generator::WorldGenerator;
use bevy::prelude::*;
use std::collections::HashSet;

use super::chunk::CHUNK_SIZE;
pub const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;
pub const RENDER_RADIUS: i32 = 16;
pub const DIST_TO_LOAD_CHUNK: i32 = RENDER_RADIUS * CHUNK_SIZE_I32;
pub const UNLOAD_RADIUS: i32 = 20;
pub const DIST_TO_UNLOAD_CHUNK: i32 = UNLOAD_RADIUS * CHUNK_SIZE_I32;

#[derive(Component)]
pub struct Dimention;

#[derive(Component)]
pub struct World {
    seed: u32,
    pub world_generator: WorldGenerator,
    pub loaded_chunks: HashSet<(i32, i32, i32)>,
    pub chunks_in_generation: HashSet<(i32, i32, i32)>,
    pub chunks_to_render: HashSet<(i32, i32, i32)>,
}

impl World {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            world_generator: WorldGenerator::new(seed),
            loaded_chunks: HashSet::new(),
            chunks_in_generation: HashSet::new(),
            chunks_to_render: HashSet::new(),
        }
    }
}

pub fn get_neighbor_chunks(chunk_coords: &(i32, i32, i32)) -> HashSet<(i32, i32, i32)> {
    let x = chunk_coords.0;
    let z = chunk_coords.2;
    HashSet::from([
        (x + CHUNK_SIZE_I32, 0, z),
        (x - CHUNK_SIZE_I32, 0, z),
        (x, 0, z + CHUNK_SIZE_I32),
        (x, 0, z - CHUNK_SIZE_I32),
    ])
}

fn spawn_world_system(mut commands: Commands) {
    commands.spawn(World::new(0)).insert(SpatialBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
}

impl Plugin for Dimention {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_world_system)
            .add_system(manage_chunks_system)
            .add_system(handle_generated_chunks_system)
            .add_system(generate_chunk_mesh_system.after(manage_chunks_system));
    }
}
