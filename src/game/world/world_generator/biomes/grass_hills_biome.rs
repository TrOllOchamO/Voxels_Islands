use noise::NoiseFn;

use crate::game::world::block::{Block, BlockOrientation};
use crate::game::world::chunk::{AbleToGenerateChunk, Chunk};
use crate::game::world::chunk_renderer::blocks_resources::blocks_ids::{
    AIR_BLOCK, GRASS_BLOCK, SAND_BLOCK, WATER_BLOCK,
};
use crate::game::world::coordinates::block_coords_in_chunk::BlockCoordsInChunk;
use crate::game::world::coordinates::global_coordinates::GlobalCoordinates;
use crate::game::world::world_generator::generator::WorldGenNoises;

use super::biome::OCEAN_LEVEL;

#[derive(Clone)]
pub struct GrassHillsBiome;

impl AbleToGenerateChunk for GrassHillsBiome {
    fn generate_chunk(&self, chunk: &mut Chunk, noises: WorldGenNoises) {
        let chunk_coords = chunk.get_coords();
        let blocks = chunk.as_mut_slice();

        for (i, mut block) in blocks.iter_mut().enumerate() {
            let pos_in_chunk = BlockCoordsInChunk::from_block_index(i);
            let g_pos = GlobalCoordinates::from_block_coordinates(&chunk_coords, &pos_in_chunk);

            let mut coord_array = g_pos.to_2d_f64_array();
            coord_array[0] /= 200.0;
            coord_array[1] /= 200.0;
            let noise_value = noises.super_simplex.get(coord_array) * 0.5 + 0.5;

            if g_pos.get_y() as f64 > noise_value * 256. {
                if g_pos.get_y() <= OCEAN_LEVEL {
                    block.0 = Block::new(WATER_BLOCK, BlockOrientation::PositiveX).0;
                } else {
                    block.0 = Block::new(AIR_BLOCK, BlockOrientation::PositiveX).0;
                }
            } else {
                if g_pos.get_y() >= OCEAN_LEVEL + 2 {
                    block.0 = Block::new(GRASS_BLOCK, BlockOrientation::PositiveX).0;
                } else {
                    block.0 = Block::new(SAND_BLOCK, BlockOrientation::PositiveX).0;
                }
            }
        }
    }
}
