use bevy::prelude::*;

pub mod blocks {
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

// WIP -------------------------------------
#[derive(Resource)]
struct BlocksResource(Vec<BlockInfos>);

#[macro_export]
macro_rules! create_blocks_resource {
    ($($block_infos:expr), +) => {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push(&block_infos);
        )+
        temp_vec
    };
}
// WIP -------------------------------------

pub fn block_is_transparent(block_id: u16) -> bool {
    get_block_infos(block_id).is_transparent
}

pub fn block_color(block_id: u16) -> Option<Color> {
    get_block_infos(block_id).color
}

#[inline(always)]
pub fn get_block_infos(block_id: u16) -> BlockInfos {
    match block_id {
        blocks::AIR_BLOCK => BlockInfos {
            id: blocks::AIR_BLOCK,
            name: "air_block",
            is_transparent: true,
            color: None,
        },
        blocks::DEBUG_BLOCK => BlockInfos {
            id: blocks::DEBUG_BLOCK,
            name: "debug_block",
            is_transparent: false,
            color: Some(Color::hsla(305., 1., 0.5, 1.)),
        },
        blocks::DIRT_BLOCK => BlockInfos {
            id: blocks::DIRT_BLOCK,
            name: "dirt_block",
            is_transparent: false,
            color: Some(Color::hsla(42., 0.82, 0.3, 1.)),
        },
        blocks::GRASS_BLOCK => BlockInfos {
            id: blocks::GRASS_BLOCK,
            name: "grass_block",
            is_transparent: false,
            color: Some(Color::hsla(127., 0.8, 0.39, 1.)),
        },
        blocks::STONE_BLOCK => BlockInfos {
            id: blocks::STONE_BLOCK,
            name: "stone_block",
            is_transparent: false,
            color: Some(Color::hsla(195., 0.02, 0.46, 1.)),
        },
        blocks::SAND_BLOCK => BlockInfos {
            id: blocks::SAND_BLOCK,
            name: "sand_block",
            is_transparent: false,
            color: Some(Color::hsla(64., 0.63, 0.62, 1.)),
        },
        blocks::WATER_BLOCK => BlockInfos {
            id: blocks::WATER_BLOCK,
            name: "water_block",
            is_transparent: true,
            color: Some(Color::hsla(204., 0.86, 0.48, 0.3)),
        },
        _ => panic!("The block id : {} does_not exist", block_id),
    }
}

pub fn get_block_id(block_name: &str) -> u16 {
    match block_name {
        "air_block" => blocks::AIR_BLOCK,
        "debug_block" => blocks::DEBUG_BLOCK,
        "dirt_block" => blocks::DIRT_BLOCK,
        "grass_block" => blocks::GRASS_BLOCK,
        "stone_block" => blocks::STONE_BLOCK,
        "sand_block" => blocks::SAND_BLOCK,
        "water_block" => blocks::WATER_BLOCK,
        _ => panic!("The block name : {} does_not exist", block_name),
    }
}
