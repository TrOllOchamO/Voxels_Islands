use super::block::{Block, BlockOrientation};

use super::chunk_renderer::blocks_resources::{AIR_BLOCK_ID, DEBUG_BLOCK_ID};
use bevy::prelude::*;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const NB_BLOCKS_PER_CHUNK: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

#[derive(Clone, Default)]
pub struct ChunkCoordinates {
    x: i32,
    y: i32,
    z: i32,
}

impl ChunkCoordinates {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        ChunkCoordinates { x, y, z }
    }

    pub fn to_float_tuple(&self) -> (f32, f32, f32) {
        (self.x as f32, self.y as f32, self.z as f32)
    }

    pub fn to_bevy_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        }
    }

    pub fn to_tuple(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}

#[derive(Clone)]
pub struct ChunkData(Vec<Block>);

impl Default for ChunkData {
    fn default() -> Self {
        let chunk_data = vec![Block::default(); NB_BLOCKS_PER_CHUNK];
        Self(chunk_data)
    }
}

impl ChunkData {
    pub fn get_block(&self, index: usize) -> Block {
        self.0[index]
    }

    pub fn as_slice(&self) -> &[Block] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [Block] {
        self.0.as_mut_slice()
    }
}

#[derive(Component, Default)]
pub struct Chunk {
    coords: ChunkCoordinates,
    pub blocks: ChunkData,
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Chunk {
            coords: ChunkCoordinates { x, y, z },
            blocks: ChunkData::default(),
        }
    }

    pub fn get_coords(&self) -> ChunkCoordinates {
        self.coords.clone()
    }

    pub fn as_slice(&self) -> &[Block] {
        self.blocks.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [Block] {
        self.blocks.as_mut_slice()
    }

    pub fn get_block(&self, index: usize) -> Block {
        self.blocks.get_block(index)
    }
}

// The position is stored like this
// XXXX YYYY YYYY ZZZZ
pub struct BlockPosition(u16);

impl BlockPosition {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        assert!(x < CHUNK_SIZE);
        assert!(z < CHUNK_SIZE);

        let mut data: u16 = x as u16;
        data = (data << 8) | (y as u16);
        data = (data << 4) | (z as u16);

        Self(data)
    }

    pub fn from_block_index(block_index: usize) -> Self {
        let x = block_index / (CHUNK_SIZE * CHUNK_HEIGHT);
        let index = block_index - x * (CHUNK_SIZE * CHUNK_HEIGHT);
        let y = index / CHUNK_SIZE;
        let z = index % CHUNK_SIZE;
        BlockPosition::new(x, y, z)
    }

    pub fn get_x(&self) -> u8 {
        (self.0 >> 12) as u8
    }

    pub fn get_y(&self) -> u8 {
        ((self.0 << 4) >> 8) as u8
    }

    pub fn get_z(&self) -> u8 {
        (self.0 & 0xF) as u8
    }

    pub fn pos_tuple(&self) -> (u8, u8, u8) {
        (self.get_x(), self.get_y(), self.get_z())
    }

    pub fn pos_tuple_u32(&self) -> (u32, u32, u32) {
        (
            self.get_x() as u32,
            self.get_y() as u32,
            self.get_z() as u32,
        )
    }

    pub fn pos_tuple_f32(&self) -> (f32, f32, f32) {
        (
            self.get_x() as f32,
            self.get_y() as f32,
            self.get_z() as f32,
        )
    }

    pub fn get_as_u16(&self) -> u16 {
        self.0
    }

    pub fn get_as_u32(&self) -> u32 {
        self.0 as u32
    }
}
