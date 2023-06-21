use std::i32::MIN;

use super::block_coords_in_chunk::BlockCoordsInChunk;
use super::chunk_coordinates::ChunkCoordinates;

pub struct GlobalCoordinates {
    x: u32,
    y: u32,
    z: u32,
}

impl GlobalCoordinates {
    pub fn to_f64_array(&self) -> [f64; 3] {
        [self.x as f64, self.y as f64, self.z as f64]
    }

    pub fn to_2d_f64_array(&self) -> [f64; 2] {
        [self.x as f64, self.z as f64]
    }

    pub fn from_block_coordinates(
        parent_chunk_coords: &ChunkCoordinates,
        block_pos: &BlockCoordsInChunk,
    ) -> Self {
        let (x, y, z) = parent_chunk_coords.to_tuple();

        // offset so negatives chunks can be represented inside a u32 coord
        let parent_x: u32 = ((x as i64) - (MIN as i64)) as u32;
        let parent_y: u32 = y as u32;
        let parent_z: u32 = ((z as i64) - (MIN as i64)) as u32;

        let (block_x, block_y, block_z) = block_pos.pos_tuple_u32();

        Self {
            x: parent_x + block_x,
            y: parent_y + block_y,
            z: parent_z + block_z,
        }
    }

    #[inline]
    pub fn get_x(&self) -> u32 {
        self.x
    }

    #[inline]
    pub fn get_y(&self) -> u32 {
        self.y
    }

    #[inline]
    pub fn get_z(&self) -> u32 {
        self.z
    }
}
