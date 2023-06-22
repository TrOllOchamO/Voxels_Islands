use crate::game::world::coordinates::block_coords_in_biome::BlockCoordsInBiome;

pub trait CanErode {
    // return a value between 0.0 and 1.0
    //
    fn erode(block_coords: &BlockCoordsInBiome) -> f64;
}
