use super::biomes::biome::Biome;
use super::chunk::AbleToGenerateChunk;
use super::{
    biomes::{desert_biome::DesertBiome, grass_hills_biome::GrassHillsBiome},
    chunk::Chunk,
};
use crate::game::world::coordinates::biome_coordinates::BiomeCoordinates;
use noise::SuperSimplex;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
        let mut hasher = DefaultHasher::new();
        biome_coords.hash(&mut hasher);

        // replace the hard coded value by this when it will be stable
        // mem::variant_count::<Biome>() as u64
        match hasher.finish() % 2 {
            0 => Biome::Desert(DesertBiome),
            1 => Biome::GrassHills(GrassHillsBiome),
            _ => unreachable!(),
        }
    }

    pub fn generate_chunk(&self, chunk: &mut Chunk) {
        let biome_coords = BiomeCoordinates::from_chunk_coords(&chunk.get_coords());
        let biome = Self::get_biome_at(&biome_coords);
        biome.generate_chunk(chunk, self.noises.clone());
    }
}
