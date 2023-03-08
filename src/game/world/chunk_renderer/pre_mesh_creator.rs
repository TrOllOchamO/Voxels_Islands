use super::block::Block;
use super::blocks_resources::blocks_ids::AIR_BLOCK;
use super::chunk::{ChunkData, CHUNK_HEIGHT, CHUNK_SIZE, NB_BLOCKS_PER_CHUNK};
use super::covered_faces::*;
use super::greedy_mesh::*;

pub const NB_BLOCK_ON_CHUNK_SIDE: usize = CHUNK_SIZE * CHUNK_HEIGHT;
pub const NB_BLOCK_ON_CHUNK_WITHOUT_ONE_SIDE: usize = NB_BLOCKS_PER_CHUNK - NB_BLOCK_ON_CHUNK_SIDE;

// the stretch contain how much a mesh should expand along the 3 axis
// The data is stored like this
// XXXX YYYY YYYY ZZZZ
#[derive(Clone, Copy, Debug)]
pub struct Stretch(u16);

impl Stretch {
    pub fn new(data: u16) -> Self {
        Self(data)
    }

    pub fn from_xyz(x: u8, y: u8, z: u8) -> Self {
        let mut data: u16 = x as u16;
        data <<= 8;
        data |= y as u16;
        data <<= 4;
        data |= z as u16;
        Self(data)
    }

    pub fn get_x(&self) -> u8 {
        (self.0 >> 12) as u8
    }

    pub fn get_y(&self) -> u8 {
        ((self.0 << 4) >> 8) as u8
    }

    pub fn set_x(&mut self, x: u8) {
        self.0 &= 0x0FFF;
        self.0 |= (x as u16) << 12;
    }

    pub fn set_y(&mut self, y: u8) {
        self.0 &= 0xF00F;
        self.0 |= (y as u16) << 4;
    }

    pub fn set_z(&mut self, z: u8) {
        self.0 &= 0xFFF0;
        self.0 |= z as u16;
    }

    pub fn get_z(&self) -> u8 {
        (self.0 & 0xF) as u8
    }

    pub fn get_as_u16(&self) -> u16 {
        self.0
    }

    pub fn get_as_u32(&self) -> u32 {
        self.0 as u32
    }

    pub fn get_final_stretch(&self) -> (f32, f32, f32) {
        let stretch_x = self.get_x();
        let stretch_y = self.get_y();
        let stretch_z = self.get_z();

        let x = if stretch_x == 0 {
            CHUNK_SIZE as f32
        } else {
            stretch_x as f32
        };
        let y = if stretch_y == 0 {
            CHUNK_HEIGHT as f32
        } else {
            stretch_y as f32
        };
        let z = if stretch_z == 0 {
            CHUNK_SIZE as f32
        } else {
            stretch_z as f32
        };

        (x, y, z)
    }
}

impl Default for Stretch {
    fn default() -> Self {
        Self(0x1011)
    }
}

// this struct contain all the data needed to create an optimaized mesh of a chunk
// Stored like this :
//IIII IIII IIIU UOOO XXXX YYYY YYYY ZZZZ
// 11 bits for the block ID marked as "I"
// 3 bits for the block orientation marked as "O"
// 4 bits for the number of blocks the mesh sould expand along the X axis marked as "X"
// 8 bits for the number of blocks the mesh sould expand along the Y axis marked as "Y"
// 4 bits for the number of blocks the mesh sould expand along the Z axis marked as "Z"
// 2 bits unused marked as "U"
#[derive(Clone, Copy, Debug)]
pub struct BlockPreMesh(u32);

impl Default for BlockPreMesh {
    fn default() -> Self {
        Self(0)
    }
}

impl BlockPreMesh {
    pub fn new(block: Block, stretch: Stretch) -> Self {
        let block_data = block.get_as_u16();
        let first_part: u32 = (block_data as u32) << 16;
        let final_data: u32 = first_part | stretch.get_as_u32();
        BlockPreMesh(final_data)
    }

    pub fn get_block(&self) -> Block {
        Block((self.0 >> 16) as u16)
    }

    pub fn get_stretch(&self) -> Stretch {
        Stretch((self.0 & 0x0000_FFFF) as u16)
    }

    pub fn should_not_be_rendered(&mut self) {
        self.0 &= 0xFFFF_0000;
    }

    pub fn will_be_rendered(&self) -> bool {
        (self.0 & 0x0000_FFFF) != 0
    }
}

#[derive(Clone)]
pub struct ChunkPreMeshOneDirection(Vec<BlockPreMesh>);

impl ChunkPreMeshOneDirection {
    pub fn new(chunk_data: &[Block]) -> Self {
        let mut blocks_pre_mesh_data = Vec::with_capacity(NB_BLOCKS_PER_CHUNK);

        for i in 0..NB_BLOCKS_PER_CHUNK {
            let block = chunk_data[i];

            let mut stretch = Stretch::default();

            if block.get_id() == AIR_BLOCK {
                stretch.0 = 0;
            }

            blocks_pre_mesh_data.push(BlockPreMesh::new(block, stretch));
        }

        Self(blocks_pre_mesh_data)
    }

    pub fn as_slice(&self) -> &[BlockPreMesh] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [BlockPreMesh] {
        self.0.as_mut_slice()
    }

    pub fn get_pre_mesh(&self, pre_mesh_index: usize) -> BlockPreMesh {
        self.0[pre_mesh_index]
    }

    pub fn set_pre_mesh(&mut self, pre_mesh_index: usize, value: BlockPreMesh) {
        self.0[pre_mesh_index] = value;
    }

    pub fn should_not_be_rendered(&mut self, pre_mesh_index: usize) {
        self.0[pre_mesh_index].should_not_be_rendered();
    }
}

pub struct NeighborChunks {
    pub positive_x: Option<ChunkData>,
    pub negative_x: Option<ChunkData>,
    pub positive_z: Option<ChunkData>,
    pub negative_z: Option<ChunkData>,
}

pub struct ChunkPreMesh {
    pub positive_x: ChunkPreMeshOneDirection,
    pub negative_x: ChunkPreMeshOneDirection,
    pub positive_y: ChunkPreMeshOneDirection,
    pub negative_y: ChunkPreMeshOneDirection,
    pub positive_z: ChunkPreMeshOneDirection,
    pub negative_z: ChunkPreMeshOneDirection,
}

impl ChunkPreMesh {
    pub fn new(chunk_data: &[Block]) -> Self {
        let positive_x = ChunkPreMeshOneDirection::new(chunk_data);
        let negative_x = positive_x.clone();
        let positive_y = positive_x.clone();
        let negative_y = positive_x.clone();
        let positive_z = positive_x.clone();
        let negative_z = positive_x.clone();

        Self {
            positive_x,
            negative_x,
            positive_y,
            negative_y,
            positive_z,
            negative_z,
        }
    }

    pub fn optimise(&mut self, neighbor_chunks: &NeighborChunks) {
        self.hide_covered_faces(neighbor_chunks);
        self.apply_greedy_meshing();
    }

    fn hide_covered_faces(&mut self, neighbor_chunks: &NeighborChunks) {
        pre_mesh_block_faces_on_positive_x(&mut self.positive_x, &neighbor_chunks.positive_x);
        pre_mesh_block_faces_on_negative_x(&mut self.negative_x, &neighbor_chunks.negative_x);
        pre_mesh_block_faces_on_positive_y(&mut self.positive_y);
        pre_mesh_block_faces_on_negative_y(&mut self.negative_y);
        pre_mesh_block_faces_on_positive_z(&mut self.positive_z, &neighbor_chunks.positive_z);
        pre_mesh_block_faces_on_negative_z(&mut self.negative_z, &neighbor_chunks.negative_z);
    }

    fn apply_greedy_meshing(&mut self) {
        merge_faces_on_x_axis(&mut self.positive_x, &mut self.negative_x);
        merge_faces_on_y_axis(&mut self.positive_y, &mut self.negative_y);
        merge_faces_on_z_axis(&mut self.positive_z, &mut self.negative_z);
    }
}
