use super::desert_biome::DesertBiome;
use super::grass_hills_biome::GrassHillsBiome;
use crate::game::world::{
    chunk::{AbleToGenerateChunk, Chunk, CHUNK_SIZE},
    world_generator::generator::WorldGenNoises,
};

pub const BIOME_SIZE_IN_CHUNKS: usize = 128;
pub const BIOME_SIZE_IN_BLOCKS: usize = CHUNK_SIZE * BIOME_SIZE_IN_CHUNKS;
pub const OCEAN_LEVEL: u32 = 48;

#[derive(Clone)]
pub enum Biome {
    Desert(DesertBiome),
    GrassHills(GrassHillsBiome),
}

impl AbleToGenerateChunk for Biome {
    fn generate_chunk(&self, chunk: &mut Chunk, noises: WorldGenNoises) {
        match self {
            Biome::Desert(desert) => desert.generate_chunk(chunk, noises),
            Biome::GrassHills(grass_hills) => grass_hills.generate_chunk(chunk, noises),
        }
    }
}
