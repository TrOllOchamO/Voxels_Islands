use super::biomes::biome::Biome;
use super::chunk::AbleToGenerateChunk;
use super::{biomes::desert_biome::DesertBiome, chunk::Chunk};
use crate::game::world::coordinates::biome_coordinates::BiomeCoordinates;
use noise::SuperSimplex;

#[derive(Clone)]
pub struct WorldGenNoises {
    pub super_simplex: SuperSimplex,
}

impl WorldGenNoises {
    pub fn new(seed: u32) -> Self {
        Self {
            super_simplex: SuperSimplex::new(seed),
        }
    }
}

#[derive(Clone)]
pub struct WorldGenerator {
    noises: WorldGenNoises,
}

impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            noises: WorldGenNoises::new(seed),
        }
    }

    fn get_biome_at(biome_coords: &BiomeCoordinates) -> Biome {
        // TODO a match when we will have more than one biome
        Biome::Desert(DesertBiome)
    }

    pub fn generate_chunk(&self, chunk: &mut Chunk) {
        let biome_coords = BiomeCoordinates::from_chunk_coords(&chunk.get_coords());
        let biome = Self::get_biome_at(&biome_coords);
        biome.generate_chunk(chunk, self.noises.clone());
    }
}
