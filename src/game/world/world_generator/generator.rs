use super::chunk::{BlockPosition, Chunk, ChunkCoordinates};
use crate::game::world::block::{Block, BlockOrientation};
use crate::game::world::chunk_renderer::blocks_resources::blocks_ids::{
    AIR_BLOCK, GRASS_BLOCK, SAND_BLOCK, WATER_BLOCK,
};
use noise::{NoiseFn, Perlin};
use std::i32::MIN;

#[derive(Clone)]
pub struct WorldGenerator {
    perlin: Perlin,
}

impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
        }
    }

    pub fn generate_chunk(&self, chunk: &mut Chunk) {
        let chunk_coords = chunk.get_coords();
        let (chunk_x, chunk_z) = convert_chunk_coords_to_noise_coords(&chunk_coords);
        let blocks = chunk.as_mut_slice();

        for (i, mut block) in blocks.iter_mut().enumerate() {
            let pos = BlockPosition::from_block_index(i);
            let noise_coords = get_noise_coords(&pos, chunk_x, chunk_z);
            let noise_value = self.perlin.get(noise_coords);
            if pos.get_y() as f64 / 256. > noise_value {
                if pos.get_y() <= 64 {
                    block.0 = Block::new(WATER_BLOCK, BlockOrientation::PositiveX).0;
                } else {
                    block.0 = Block::new(AIR_BLOCK, BlockOrientation::PositiveX).0;
                }
            } else {
                if pos.get_y() >= 66 {
                    block.0 = Block::new(GRASS_BLOCK, BlockOrientation::PositiveX).0;
                } else {
                    block.0 = Block::new(SAND_BLOCK, BlockOrientation::PositiveX).0;
                }
            }
        }
    }
}

fn convert_chunk_coords_to_noise_coords(chunk_coords: &ChunkCoordinates) -> (u32, u32) {
    let (x, _, z) = chunk_coords.to_tuple();
    let u_x: u32 = ((x as i64) - (MIN as i64)) as u32;
    let u_z: u32 = ((z as i64) - (MIN as i64)) as u32;
    (u_x, u_z)
}

fn get_noise_coords(block_pos: &BlockPosition, chunk_x: u32, chunk_z: u32) -> [f64; 2] {
    let (block_x, _, block_z) = block_pos.pos_tuple_u32();
    let x = (block_x + chunk_x) as f64;
    let z = (block_z + chunk_z) as f64;
    [x / 100., z / 100.]
}
