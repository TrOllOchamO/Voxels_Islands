use noise::{NoiseFn, ScaleBias, SuperSimplex};

use crate::game::world::block::{Block, BlockOrientation};
use crate::game::world::chunk::{AbleToGenerateChunk, Chunk, CHUNK_HEIGHT};
use crate::game::world::chunk_renderer::blocks_resources::blocks_ids::{
    AIR_BLOCK, SAND_BLOCK, WATER_BLOCK,
};
use crate::game::world::coordinates::block_coords_in_chunk::BlockCoordsInChunk;
use crate::game::world::coordinates::global_coordinates::GlobalCoordinates;
use crate::game::world::world_generator::generator::WorldGenNoises;

use super::biome::OCEAN_LEVEL;

#[derive(Clone)]
pub struct DesertBiome;

impl AbleToGenerateChunk for DesertBiome {
    fn generate_chunk(&self, chunk: &mut Chunk, noises: WorldGenNoises) {
        let chunk_coords = chunk.get_coords();
        let blocks = chunk.as_mut_slice();
        let mut scale = 0.5; // multiply by 0.5 to set the range in [-0.5; 0.5]
        let mut bias = 0.5; // offset by 0.5 to push the range to [0;1]
        bias += OCEAN_LEVEL as f64 / CHUNK_HEIGHT as f64; // offset the land level so there is no laque of water on the land
        scale -= 0.35; // flatten the relief so it looks more like dunes

        let desert_noise: ScaleBias<f64, SuperSimplex, 2> = ScaleBias::new(noises.super_simplex)
            .set_scale(scale)
            .set_bias(bias);

        for (i, mut block) in blocks.iter_mut().enumerate() {
            let pos_in_chunk = BlockCoordsInChunk::from_block_index(i);
            let g_pos = GlobalCoordinates::from_block_coordinates(&chunk_coords, &pos_in_chunk);

            let mut coord_array = g_pos.to_2d_f64_array();
            coord_array[0] /= 300.0; // asymetric stretching so it produce a dune pushed by the wind like shape
            coord_array[1] /= 150.0;
            let noise_value = desert_noise.get(coord_array);

            // TODO erosion

            if g_pos.get_y() as f64 > noise_value * 256. {
                if g_pos.get_y() <= OCEAN_LEVEL {
                    block.0 = Block::new(WATER_BLOCK, BlockOrientation::PositiveX).0;
                } else {
                    block.0 = Block::new(AIR_BLOCK, BlockOrientation::PositiveX).0;
                }
            } else {
                block.0 = Block::new(SAND_BLOCK, BlockOrientation::PositiveX).0;
            }
        }
    }
}
