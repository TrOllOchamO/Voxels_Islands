use std::i32::MIN;

use super::chunk_coordinates::ChunkCoordinates;
use crate::game::world::world_generator::biomes::biome::BIOME_SIZE_IN_BLOCKS;

#[derive(Hash)]
pub struct BiomeCoordinates {
    x: u32,
    z: u32,
}

impl BiomeCoordinates {
    pub fn from_chunk_coords(chunk_coords: &ChunkCoordinates) -> Self {
        let (chunk_x, _, chunk_z) = chunk_coords.to_tuple();

        // offset coordinates so it become global coordinates
        let global_x = ((chunk_x as i64) - (MIN as i64)) as u32;
        let global_z = ((chunk_z as i64) - (MIN as i64)) as u32;

        // only keep a multiple of the biome size as coordinates
        Self {
            x: global_x - (global_x % BIOME_SIZE_IN_BLOCKS as u32),
            z: global_z - (global_z % BIOME_SIZE_IN_BLOCKS as u32),
        }
    }
}
