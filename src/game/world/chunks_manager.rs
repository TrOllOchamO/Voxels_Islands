use super::chunk::Chunk;
use super::world::get_neighbor_chunks;
use super::world::World;
use super::world::CHUNK_SIZE_I32;
use super::world::DIST_TO_LOAD_CHUNK;
use super::world::DIST_TO_UNLOAD_CHUNK;
use super::world::RENDER_RADIUS;
use crate::game::camera::{FirstPersonCamera, Position};
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use std::collections::{HashMap, HashSet};

const NB_CHUNKS_MAX_GENERATING_IN_THE_BACKGROUND: usize = 1;

#[derive(Bundle)]
struct LoadChunkBundle {
    chunk: Chunk,
    transform: SpatialBundle,
}

#[derive(Component)]
pub struct ComputeChunk {
    task: Task<Chunk>,
    parent_world: Entity,
}

pub fn manage_chunks_system(
    mut commands: Commands,
    mut worlds: Query<(Entity, &mut World)>,
    chunks: Query<(Entity, &mut Chunk)>,
    cameras_pos: Query<&Position, With<FirstPersonCamera>>,
) {
    for (world_entity, mut world) in worlds.iter_mut() {
        let chunks_to_unload = get_chunks_to_unload(&world, &cameras_pos);
        unload_chunks(&mut world, &mut commands, &chunks, &chunks_to_unload);
        let chunks_to_load = get_chunks_to_load(&world, &cameras_pos);
        let mut rated_chunks = rate_loading_priority(&chunks_to_load, &cameras_pos);
        load_chunks(&mut world, world_entity, &mut commands, &mut rated_chunks);

        start_generating_chunks(&mut world, world_entity, &mut commands, &rated_chunks);
    }
}

fn get_chunks_to_unload(
    world: &World,
    cameras_pos: &Query<&Position, With<FirstPersonCamera>>,
) -> HashSet<(i32, i32, i32)> {
    let mut chunks_to_unload = world.loaded_chunks.clone();
    for camera_pos in cameras_pos.iter() {
        chunks_to_unload.retain(|chunk| -> bool {
            let diff_x_squared = (camera_pos.x as i32 - chunk.0).pow(2) as f32;
            let diff_z_squared = (camera_pos.z as i32 - chunk.2).pow(2) as f32;
            let dist = f32::sqrt(diff_x_squared + diff_z_squared) as i32;
            dist > DIST_TO_UNLOAD_CHUNK
        });
    }
    chunks_to_unload
}

fn unload_chunks(
    world: &mut World,
    commands: &mut Commands,
    chunks: &Query<(Entity, &mut Chunk)>,
    chunks_to_unload: &HashSet<(i32, i32, i32)>,
) {
    for (chunk_entity, chunk) in chunks.iter() {
        let chunk_coords = chunk.get_coords().to_tuple();
        if chunks_to_unload.contains(&chunk_coords) {
            commands.entity(chunk_entity).despawn_recursive();
            world.loaded_chunks.remove(&chunk_coords);
            let chunks_to_rerender = get_neighbor_chunks(&chunk_coords);
            world.chunks_to_render.extend(chunks_to_rerender);
            world.chunks_to_render.remove(&chunk_coords);
        }
    }
}

fn get_chunks_to_load(
    world: &World,
    cameras_pos: &Query<&Position, With<FirstPersonCamera>>,
) -> HashSet<(i32, i32, i32)> {
    let mut chunks_to_load = HashSet::new();

    for camera_pos in cameras_pos.iter() {
        let camera_chunk_x = ((camera_pos.x as i32) / CHUNK_SIZE_I32) * CHUNK_SIZE_I32;
        let camera_chunk_z = ((camera_pos.z as i32) / CHUNK_SIZE_I32) * CHUNK_SIZE_I32;

        for x in -RENDER_RADIUS..RENDER_RADIUS {
            for z in -RENDER_RADIUS..RENDER_RADIUS {
                let diff_x_squared = (x * CHUNK_SIZE_I32).pow(2) as f32;
                let diff_z_squared = (z * CHUNK_SIZE_I32).pow(2) as f32;
                let dist = f32::sqrt(diff_x_squared + diff_z_squared) as i32;

                if dist < DIST_TO_LOAD_CHUNK {
                    chunks_to_load.insert((
                        camera_chunk_x + x * CHUNK_SIZE_I32,
                        0,
                        camera_chunk_z + z * CHUNK_SIZE_I32,
                    ));
                }
            }
        }
    }

    chunks_to_load.retain(|chunk_coords| -> bool { !world.loaded_chunks.contains(chunk_coords) });
    chunks_to_load
        .retain(|chunk_coords| -> bool { !world.chunks_in_generation.contains(chunk_coords) });
    chunks_to_load
}

fn rate_loading_priority(
    chunks_to_load: &HashSet<(i32, i32, i32)>,
    cameras_pos: &Query<&Position, With<FirstPersonCamera>>,
) -> HashMap<(i32, i32, i32), f32> {
    let mut rated_chunks = HashMap::new();
    for coords in chunks_to_load.iter() {
        rated_chunks.insert(*coords, 0.);
    }

    for camera in cameras_pos.iter() {
        for ((x, y, z), key_score) in rated_chunks.iter_mut() {
            let diff_x = (camera.x - *x as f32).powi(2);
            let diff_y = (camera.y - *y as f32).powi(2);
            let diff_z = (camera.z - *z as f32).powi(2);

            let dist = (diff_x + diff_y + diff_z).sqrt();
            let score = dist.sqrt();
            *key_score += score;
        }
    }

    rated_chunks
}

fn load_chunks(
    world: &mut World,
    world_entity: Entity,
    commands: &mut Commands,
    chunks_to_load: &mut HashMap<(i32, i32, i32), f32>,
) {
    // let has_already_been_generated = false;
    // for chunk in chunks_to_load.keys() {
    //     // TODO chunk saving and loding
    //     if has_already_been_generated {
    //         chunks_to_load.remove(chunk);
    //     }
    // }
}

fn start_generating_chunks(
    world: &mut World,
    world_entity: Entity,
    commands: &mut Commands,
    rated_chunks_to_generate: &HashMap<(i32, i32, i32), f32>,
) {
    let nb_chunks_in_generation = world.chunks_in_generation.len();
    if nb_chunks_in_generation >= NB_CHUNKS_MAX_GENERATING_IN_THE_BACKGROUND {
        return;
    }

    let thread_pool = AsyncComputeTaskPool::get();

    let nb_chunks_to_generate =
        NB_CHUNKS_MAX_GENERATING_IN_THE_BACKGROUND - nb_chunks_in_generation;

    let mut chunks_prioritized: Vec<(i32, i32, i32)> =
        rated_chunks_to_generate.keys().cloned().collect();
    chunks_prioritized.sort_by(|coords1, coords2| {
        let score1 = rated_chunks_to_generate.get(coords1).unwrap();
        let score2 = rated_chunks_to_generate.get(coords2).unwrap();
        score1.partial_cmp(score2).unwrap()
    });

    let mut chunks_to_generate = chunks_prioritized.iter().take(nb_chunks_to_generate);

    while let Some(&chunk_coords) = chunks_to_generate.next() {
        let (x, y, z) = chunk_coords;
        let world_generator = world.world_generator.clone();
        let task = thread_pool.spawn(async move {
            let mut chunk = Chunk::new(x, y, z);
            world_generator.generate_chunk(&mut chunk);
            chunk
        });

        world.chunks_in_generation.insert(chunk_coords.clone());

        commands.spawn(ComputeChunk {
            task,
            parent_world: world_entity,
        });
    }
}

pub fn handle_generated_chunks_system(
    mut worlds: Query<(Entity, &mut World)>,
    mut tasks: Query<(Entity, &mut ComputeChunk)>,
    mut commands: Commands,
) {
    for (chunk_entity, mut compute_struct) in tasks.iter_mut() {
        let opt = future::block_on(future::poll_once(&mut compute_struct.task));
        match opt {
            None => (),
            Some(computed_chunk) => load_generated_chunk(
                &mut worlds,
                &mut commands,
                computed_chunk,
                chunk_entity,
                compute_struct.parent_world,
            ),
        }
    }
}

fn load_generated_chunk(
    worlds: &mut Query<(Entity, &mut World)>,
    commands: &mut Commands,
    computed_chunk: Chunk,
    chunk_entity: Entity,
    chunk_parent: Entity,
) {
    let parent_world = worlds.get_mut(chunk_parent);

    match parent_world {
        Ok((_, mut world)) => {
            let computed_chunk_coords = computed_chunk.get_coords().to_tuple();
            let (x, y, z) = computed_chunk_coords;
            let load_chunk_bundle = LoadChunkBundle {
                chunk: computed_chunk,
                transform: SpatialBundle {
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                    ..Default::default()
                },
            };

            commands.entity(chunk_entity).insert(load_chunk_bundle);
            commands.entity(chunk_parent).add_child(chunk_entity);
            commands.entity(chunk_entity).remove::<ComputeChunk>();
            world.loaded_chunks.insert(computed_chunk_coords);
            let chunks_to_rerender = get_neighbor_chunks(&computed_chunk_coords);
            world.chunks_to_render.extend(chunks_to_rerender);
            world.chunks_to_render.insert(computed_chunk_coords);
            world.chunks_in_generation.remove(&computed_chunk_coords);
        }
        Err(_) => commands.entity(chunk_entity).despawn_recursive(),
    }
}
