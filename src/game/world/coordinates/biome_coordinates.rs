use super::chunk_coordinates::ChunkCoordinates;
use crate::game::world::world_generator::biomes::biome::BIOME_SIZE_IN_BLOCKS;

pub struct BiomeCoordinates {
    x: i32,
    z: i32,
}

impl BiomeCoordinates {
    pub fn from_chunk_coords(chunk_coords: &ChunkCoordinates) -> Self {
        let (c_x, _, c_z) = chunk_coords.to_tuple();
        Self {
            x: c_x / BIOME_SIZE_IN_BLOCKS as i32,
            z: c_z / BIOME_SIZE_IN_BLOCKS as i32,
        }
    }
}
