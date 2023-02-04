use bevy::prelude::*;

pub const AIR_BLOCK_ID: u16 = 0;
pub const DEBUG_BLOCK_ID: u16 = 1;
pub const DIRT_BLOCK_ID: u16 = 2;
pub const GRASS_BLOCK_ID: u16 = 3;
pub const STONE_BLOCK_ID: u16 = 4;
pub const SAND_BLOCK_ID: u16 = 5;
pub const WATER_BLOCK_ID: u16 = 6;

pub struct BlockInfos {
    pub id: u16,
    pub name: &'static str,
    pub is_transparent: bool,
    pub color: Option<Color>,
}

const NB_BLOCKS: u16 = 7;

pub const BLOCKS_TABLE: [BlockInfos; NB_BLOCKS as usize] = [
    // air block
    BlockInfos {
        id: AIR_BLOCK_ID,
        name: "air_block",
        is_transparent: true,
        color: None,
    },
    // debug block
    BlockInfos {
        id: DEBUG_BLOCK_ID,
        name: "debug_block",
        is_transparent: false,
        color: Some(Color::hsla(305., 1., 0.5, 1.)),
    },
    // dirt block
    BlockInfos {
        id: DIRT_BLOCK_ID,
        name: "dirt_block",
        is_transparent: false,
        color: Some(Color::hsla(42., 0.82, 0.3, 1.)),
    },
    // grass block
    BlockInfos {
        id: GRASS_BLOCK_ID,
        name: "grass_block",
        is_transparent: false,
        color: Some(Color::hsla(127., 0.8, 0.39, 1.)),
    },
    // stone block
    BlockInfos {
        id: STONE_BLOCK_ID,
        name: "stone_block",
        is_transparent: false,
        color: Some(Color::hsla(195., 0.02, 0.46, 1.)),
    },
    // sand block
    BlockInfos {
        id: SAND_BLOCK_ID,
        name: "sand_block",
        is_transparent: false,
        color: Some(Color::hsla(64., 0.63, 0.62, 1.)),
    },
    // water block
    BlockInfos {
        id: WATER_BLOCK_ID,
        name: "water_block",
        is_transparent: true,
        color: Some(Color::hsla(204., 0.86, 0.48, 0.3)),
    },
];
