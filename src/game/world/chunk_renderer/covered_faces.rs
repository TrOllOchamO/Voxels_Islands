use super::chunk::ChunkData;
use super::pre_mesh_creator::ChunkPreMeshOneDirection;
use super::pre_mesh_creator::{NB_BLOCK_ON_CHUNK_SIDE, NB_BLOCK_ON_CHUNK_WITHOUT_ONE_SIDE};
use super::resource_extractor::block_is_transparent;
use crate::game::world::block::Block;
use crate::game::world::chunk::CHUNK_SIZE;

pub fn pre_mesh_block_faces_on_positive_x(
    pre_mesh_data: &mut ChunkPreMeshOneDirection,
    neighbor_chunk_on_positive_x: &Option<ChunkData>,
) {
    match neighbor_chunk_on_positive_x {
        Some(chunk_data) => pre_mesh_positive_x_chunk_side(pre_mesh_data, chunk_data),
        None => (),
    }

    let array_offset = NB_BLOCK_ON_CHUNK_SIDE;

    for i in 0..NB_BLOCK_ON_CHUNK_WITHOUT_ONE_SIDE {
        let neighbor_index = i + array_offset;
        let neighbor_block = pre_mesh_data.get_pre_mesh(neighbor_index).get_block();
        let current_index = i;
        let current_block = pre_mesh_data.get_pre_mesh(current_index).get_block();

        if !face_should_be_rendered(&current_block, &neighbor_block) {
            pre_mesh_data.should_not_be_rendered(current_index);
        }
    }
}

fn pre_mesh_positive_x_chunk_side(
    current_chunk: &mut ChunkPreMeshOneDirection,
    neighbor_chunk: &ChunkData,
) {
    let array_offset = NB_BLOCK_ON_CHUNK_WITHOUT_ONE_SIDE;
    for i in 0..NB_BLOCK_ON_CHUNK_SIDE {
        let neighbor_index = i;
        let neighbor_block = neighbor_chunk.get_block(neighbor_index);
        let current_index = array_offset + i;
        let current_block = current_chunk.get_pre_mesh(current_index).get_block();

        if !face_should_be_rendered(&current_block, &neighbor_block) {
            current_chunk.should_not_be_rendered(current_index);
        }
    }
}

pub fn pre_mesh_block_faces_on_negative_x(
    pre_mesh_data: &mut ChunkPreMeshOneDirection,
    neighbor_chunk_on_negative_x: &Option<ChunkData>,
) {
    match neighbor_chunk_on_negative_x {
        Some(neighbor_chunk) => pre_mesh_negative_x_chunk_side(pre_mesh_data, neighbor_chunk),
        None => (),
    }

    let array_offset = NB_BLOCK_ON_CHUNK_SIDE;
    for i in 0..NB_BLOCK_ON_CHUNK_WITHOUT_ONE_SIDE {
        let neighbor_index = i;
        let neighbor_block = pre_mesh_data.get_pre_mesh(neighbor_index).get_block();
        let current_index = i + array_offset;
        let current_block = pre_mesh_data.get_pre_mesh(current_index).get_block();

        if !face_should_be_rendered(&current_block, &neighbor_block) {
            pre_mesh_data.should_not_be_rendered(current_index);
        }
    }
}

fn pre_mesh_negative_x_chunk_side(
    current_chunk: &mut ChunkPreMeshOneDirection,
    neighbor_chunk: &ChunkData,
) {
    let array_offset = NB_BLOCK_ON_CHUNK_WITHOUT_ONE_SIDE;
    for i in 0..NB_BLOCK_ON_CHUNK_SIDE {
        let neighbor_index = i + array_offset;
        let neighbor_block = neighbor_chunk.get_block(neighbor_index);
        let current_index = i;
        let current_block = current_chunk.get_pre_mesh(current_index).get_block();

        if !face_should_be_rendered(&current_block, &neighbor_block) {
            current_chunk.should_not_be_rendered(current_index);
        }
    }
}

pub fn pre_mesh_block_faces_on_positive_z(
    pre_mesh_data: &mut ChunkPreMeshOneDirection,
    neighbor_chunk_on_positive_z: &Option<ChunkData>,
) {
    match neighbor_chunk_on_positive_z {
        Some(chunk_data) => pre_mesh_positive_z_chunk_side(pre_mesh_data, chunk_data),
        None => (),
    }

    for i in 0..NB_BLOCK_ON_CHUNK_SIDE {
        for k in 0..(CHUNK_SIZE - 1) {
            let neighbor_index = i * CHUNK_SIZE + k + 1;
            let neighbor_block = pre_mesh_data.get_pre_mesh(neighbor_index).get_block();
            let current_index = i * CHUNK_SIZE + k;
            let current_block = pre_mesh_data.get_pre_mesh(current_index).get_block();

            if !face_should_be_rendered(&current_block, &neighbor_block) {
                pre_mesh_data.should_not_be_rendered(current_index);
            }
        }
    }
}

fn pre_mesh_positive_z_chunk_side(
    current_chunk: &mut ChunkPreMeshOneDirection,
    neighbor_chunk: &ChunkData,
) {
    for i in 0..NB_BLOCK_ON_CHUNK_SIDE {
        let neighbor_index = i * CHUNK_SIZE;
        let neighbor_block = neighbor_chunk.get_block(neighbor_index);
        let current_index = i * CHUNK_SIZE + CHUNK_SIZE - 1;
        let current_block = current_chunk.get_pre_mesh(current_index).get_block();

        if !face_should_be_rendered(&current_block, &neighbor_block) {
            current_chunk.should_not_be_rendered(current_index);
        }
    }
}

pub fn pre_mesh_block_faces_on_negative_z(
    pre_mesh_data: &mut ChunkPreMeshOneDirection,
    neighbor_chunk_on_negative_z: &Option<ChunkData>,
) {
    match neighbor_chunk_on_negative_z {
        Some(neighbor_chunk) => pre_mesh_negative_z_chunk_side(pre_mesh_data, neighbor_chunk),
        None => (),
    }

    for i in 0..NB_BLOCK_ON_CHUNK_SIDE {
        for k in 1..(CHUNK_SIZE) {
            let neighbor_index = i * CHUNK_SIZE + k - 1;
            let neighbor_block = pre_mesh_data.get_pre_mesh(neighbor_index).get_block();
            let current_index = i * CHUNK_SIZE + k;
            let current_block = pre_mesh_data.get_pre_mesh(current_index).get_block();

            if !face_should_be_rendered(&current_block, &neighbor_block) {
                pre_mesh_data.should_not_be_rendered(current_index);
            }
        }
    }
}

fn pre_mesh_negative_z_chunk_side(
    current_chunk: &mut ChunkPreMeshOneDirection,
    neighbor_chunk: &ChunkData,
) {
    for i in 0..NB_BLOCK_ON_CHUNK_SIDE {
        let neighbor_index = i * CHUNK_SIZE + CHUNK_SIZE - 1;
        let neighbor_block = neighbor_chunk.get_block(neighbor_index);
        let current_index = i * CHUNK_SIZE;
        let current_block = current_chunk.get_pre_mesh(current_index).get_block();

        if !face_should_be_rendered(&current_block, &neighbor_block) {
            current_chunk.should_not_be_rendered(current_index);
        }
    }
}

pub fn pre_mesh_block_faces_on_positive_y(pre_mesh_data: &mut ChunkPreMeshOneDirection) {
    for i in 0..CHUNK_SIZE {
        for k in 0..(NB_BLOCK_ON_CHUNK_SIDE - CHUNK_SIZE) {
            let neighbor_index = i * NB_BLOCK_ON_CHUNK_SIDE + k + CHUNK_SIZE;
            let neighbor_block = pre_mesh_data.get_pre_mesh(neighbor_index).get_block();
            let current_index = i * NB_BLOCK_ON_CHUNK_SIDE + k;
            let current_block = pre_mesh_data.get_pre_mesh(current_index).get_block();

            if !face_should_be_rendered(&current_block, &neighbor_block) {
                pre_mesh_data.should_not_be_rendered(current_index);
            }
        }
    }
}

pub fn pre_mesh_block_faces_on_negative_y(pre_mesh_data: &mut ChunkPreMeshOneDirection) {
    for i in 0..CHUNK_SIZE {
        for k in CHUNK_SIZE..NB_BLOCK_ON_CHUNK_SIDE {
            let neighbor_index = i * NB_BLOCK_ON_CHUNK_SIDE + k - CHUNK_SIZE;
            let neighbor_block = pre_mesh_data.get_pre_mesh(neighbor_index).get_block();
            let current_index = i * NB_BLOCK_ON_CHUNK_SIDE + k;
            let current_block = pre_mesh_data.get_pre_mesh(current_index).get_block();

            if !face_should_be_rendered(&current_block, &neighbor_block) {
                pre_mesh_data.should_not_be_rendered(current_index);
            }
        }
    }
}

fn face_should_be_rendered(current_block: &Block, neighbor_block: &Block) -> bool {
    if !block_is_transparent(neighbor_block.get_id()) {
        return false;
    }

    if current_block == neighbor_block {
        return false;
    }
    true
}
