use bevy::prelude::*;

pub mod Blocks {
    pub const AIR_BLOCK: u16 = 0;
    pub const DEBUG_BLOCK: u16 = 1;
    pub const DIRT_BLOCK: u16 = 2;
    pub const GRASS_BLOCK: u16 = 3;
    pub const STONE_BLOCK: u16 = 4;
    pub const SAND_BLOCK: u16 = 5;
    pub const WATER_BLOCK: u16 = 6;
}

pub struct BlockInfos {
    pub id: u16,
    pub name: &'static str,
    pub is_transparent: bool,
    pub color: Option<Color>,
}

pub fn block_is_transparent(block_id: u16) -> bool {
    get_block_infos(block_id).is_transparent
}

pub fn block_color(block_id: u16) -> Option<Color> {
    get_block_infos(block_id).color
}

#[inline(always)]
pub fn get_block_infos(block_id: u16) -> BlockInfos {
    match block_id {
        Blocks::AIR_BLOCK => BlockInfos {
            id: block_id,
            name: "air_block",
            is_transparent: true,
            color: None,
        },
        Blocks::DEBUG_BLOCK => BlockInfos {
            id: block_id,
            name: "debug_block",
            is_transparent: false,
            color: Some(Color::hsla(305., 1., 0.5, 1.)),
        },
        Blocks::DIRT_BLOCK => BlockInfos {
            id: block_id,
            name: "dirt_block",
            is_transparent: false,
            color: Some(Color::hsla(42., 0.82, 0.3, 1.)),
        },
        Blocks::GRASS_BLOCK => BlockInfos {
            id: block_id,
            name: "grass_block",
            is_transparent: false,
            color: Some(Color::hsla(127., 0.8, 0.39, 1.)),
        },
        Blocks::STONE_BLOCK => BlockInfos {
            id: block_id,
            name: "stone_block",
            is_transparent: false,
            color: Some(Color::hsla(195., 0.02, 0.46, 1.)),
        },
        Blocks::SAND_BLOCK => BlockInfos {
            id: block_id,
            name: "sand_block",
            is_transparent: false,
            color: Some(Color::hsla(64., 0.63, 0.62, 1.)),
        },
        Blocks::WATER_BLOCK => BlockInfos {
            id: block_id,
            name: "water_block",
            is_transparent: true,
            color: Some(Color::hsla(204., 0.86, 0.48, 0.3)),
        },
        _ => panic!("The block id : {} does_not exist", block_id),
    }
}
