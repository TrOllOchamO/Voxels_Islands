use crate::game::world::chunk::{CHUNK_HEIGHT, CHUNK_SIZE};

// The position is stored like this
// XXXX YYYY YYYY ZZZZ
pub struct BlockCoordsInChunk(u16);

impl BlockCoordsInChunk {
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
        Self::new(x, y, z)
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
