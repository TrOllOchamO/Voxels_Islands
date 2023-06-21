use super::block::Block;

use super::coordinates::chunk_coordinates::ChunkCoordinates;
use super::world_generator::generator::WorldGenNoises;
use bevy::prelude::*;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const NB_BLOCKS_PER_CHUNK: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

pub trait AbleToGenerateChunk {
    fn generate_chunk(&self, chunk: &mut Chunk, noises: WorldGenNoises);
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
            coords: ChunkCoordinates::new(x, y, z),
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
