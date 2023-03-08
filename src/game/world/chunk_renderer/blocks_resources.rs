use bevy::prelude::*;
use blocks_macro::create_blocks;

pub fn block_is_transparent(block_id: u16) -> bool {
    BLOCKS[block_id as usize].is_transparent
}

pub fn block_color(block_id: u16) -> Option<Color> {
    BLOCKS[block_id as usize].color
}

create_blocks!(
    AIR_BLOCK: {
        is_transparent: true,
        color: None
    },
    DEBUG_BLOCK: {
        is_transparent: false,
        color: Some(Color::hsla(305., 1., 0.5, 1.))
    },
    DIRT_BLOCK: {
        is_transparent: false,
        color: Some(Color::hsla(42., 0.82, 0.3, 1.))
    },
    GRASS_BLOCK: {
        is_transparent: false,
        color: Some(Color::hsla(127., 0.8, 0.39, 1.))
    },
    STONE_BLOCK: {
        is_transparent: false,
        color: Some(Color::hsla(195., 0.02, 0.46, 1.))
    },
    SAND_BLOCK: {
        is_transparent: false,
        color: Some(Color::hsla(64., 0.63, 0.62, 1.))
    },
    WATER_BLOCK: {
        is_transparent: true,
        color: Some(Color::hsla(204., 0.86, 0.48, 0.3))
    }
);
