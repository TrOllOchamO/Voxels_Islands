use super::blocks_resources::{BlockInfos, BLOCKS_TABLE};
use bevy::prelude::*;

pub fn block_is_transparent(block_id: u16) -> bool {
    BLOCKS_TABLE[block_id as usize].is_transparent
}

pub fn block_color(block_id: u16) -> Option<Color> {
    BLOCKS_TABLE[block_id as usize].color
}
