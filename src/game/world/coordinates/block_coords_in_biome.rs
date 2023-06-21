use super::global_coordinates::GlobalCoordinates;
use crate::game::world::world_generator::biomes::biome::BIOME_SIZE_IN_BLOCKS;

pub struct BlockCoordsInBiome {
    x: u16,
    y: u16,
    z: u16,
}

impl BlockCoordsInBiome {
    pub fn to_f64_array(&self) -> [f64; 3] {
        [self.x as f64, self.y as f64, self.z as f64]
    }

    pub fn to_2d_f64_array(&self) -> [f64; 2] {
        [self.x as f64, self.z as f64]
    }

    pub fn from_global_coordinates(coords: &GlobalCoordinates) -> Self {
        Self {
            x: (coords.get_x() % BIOME_SIZE_IN_BLOCKS as u32) as u16,
            y: (coords.get_y() % BIOME_SIZE_IN_BLOCKS as u32) as u16,
            z: (coords.get_z() % BIOME_SIZE_IN_BLOCKS as u32) as u16,
        }
    }

    #[inline]
    pub fn get_x(&self) -> u16 {
        self.x
    }

    #[inline]
    pub fn get_y(&self) -> u16 {
        self.y
    }

    #[inline]
    pub fn get_z(&self) -> u16 {
        self.z
    }
}
