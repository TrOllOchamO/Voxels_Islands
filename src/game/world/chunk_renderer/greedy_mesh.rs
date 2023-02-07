use super::chunk::{CHUNK_SIZE, NB_BLOCKS_PER_CHUNK};
use super::pre_mesh_creator::{
    BlockPreMesh, ChunkPreMeshOneDirection, Stretch, NB_BLOCK_ON_CHUNK_SIDE,
};

pub fn merge_faces_on_x_axis(
    positive_x_pre_mesh: &mut ChunkPreMeshOneDirection,
    negative_x_pre_mesh: &mut ChunkPreMeshOneDirection,
) {
    for i in 0..NB_BLOCKS_PER_CHUNK {
        if i % CHUNK_SIZE == 0 {
            continue;
        }

        merge_faces_on_x_axis_along_z(positive_x_pre_mesh, i);
        merge_faces_on_x_axis_along_z(negative_x_pre_mesh, i);
    }

    for i in 0..NB_BLOCKS_PER_CHUNK {
        if i % NB_BLOCK_ON_CHUNK_SIDE < CHUNK_SIZE {
            continue;
        }

        merge_faces_on_x_axis_along_y(positive_x_pre_mesh, i);
        merge_faces_on_x_axis_along_y(negative_x_pre_mesh, i);
    }
}

fn merge_faces_on_x_axis_along_z(pre_mesh_on_x_axis: &mut ChunkPreMeshOneDirection, index: usize) {
    let pre_mesh_1 = pre_mesh_on_x_axis.get_pre_mesh(index - 1);
    let pre_mesh_2 = pre_mesh_on_x_axis.get_pre_mesh(index);

    if !pre_mesh_1.will_be_rendered() || !pre_mesh_2.will_be_rendered() {
        return;
    }

    if pre_mesh_1.get_block() != pre_mesh_2.get_block() {
        return;
    }

    let pre_mesh_1_stretch = pre_mesh_1.get_stretch();
    let pre_mesh_2_stretch = pre_mesh_2.get_stretch();

    if pre_mesh_1_stretch.get_y() != pre_mesh_2_stretch.get_y() {
        return;
    }

    let new_block = pre_mesh_2.get_block();
    let new_stretch = Stretch::from_xyz(
        1,
        pre_mesh_1_stretch.get_y(),
        pre_mesh_1_stretch.get_z().wrapping_add(1),
    );

    let new_pre_mesh = BlockPreMesh::new(new_block, new_stretch);
    pre_mesh_on_x_axis.set_pre_mesh(index, new_pre_mesh);
    pre_mesh_on_x_axis.should_not_be_rendered(index - 1);
}

fn merge_faces_on_x_axis_along_y(pre_mesh_on_x_axis: &mut ChunkPreMeshOneDirection, index: usize) {
    let pre_mesh_1 = pre_mesh_on_x_axis.get_pre_mesh(index - CHUNK_SIZE);
    let pre_mesh_2 = pre_mesh_on_x_axis.get_pre_mesh(index);

    if !pre_mesh_1.will_be_rendered() || !pre_mesh_2.will_be_rendered() {
        return;
    }

    if pre_mesh_1.get_block() != pre_mesh_2.get_block() {
        return;
    }

    let pre_mesh_1_stretch = pre_mesh_1.get_stretch();
    let pre_mesh_2_stretch = pre_mesh_2.get_stretch();

    if pre_mesh_1_stretch.get_z() != pre_mesh_2_stretch.get_z() {
        return;
    }

    let new_block = pre_mesh_2.get_block();
    let new_stretch = Stretch::from_xyz(
        1,
        pre_mesh_1_stretch.get_y().wrapping_add(1),
        pre_mesh_1_stretch.get_z(),
    );
    let new_pre_mesh = BlockPreMesh::new(new_block, new_stretch);
    pre_mesh_on_x_axis.set_pre_mesh(index, new_pre_mesh);
    pre_mesh_on_x_axis.should_not_be_rendered(index - CHUNK_SIZE);
}

pub fn merge_faces_on_z_axis(
    positive_z_pre_mesh: &mut ChunkPreMeshOneDirection,
    negative_z_pre_mesh: &mut ChunkPreMeshOneDirection,
) {
    for i in NB_BLOCK_ON_CHUNK_SIDE..NB_BLOCKS_PER_CHUNK {
        merge_faces_on_z_axis_along_x(positive_z_pre_mesh, i);
        merge_faces_on_z_axis_along_x(negative_z_pre_mesh, i);
    }

    for i in 0..NB_BLOCKS_PER_CHUNK {
        if i % NB_BLOCK_ON_CHUNK_SIDE < CHUNK_SIZE {
            continue;
        }

        merge_faces_on_z_axis_along_y(positive_z_pre_mesh, i);
        merge_faces_on_z_axis_along_y(negative_z_pre_mesh, i);
    }
}

fn merge_faces_on_z_axis_along_x(pre_mesh_on_z_axis: &mut ChunkPreMeshOneDirection, index: usize) {
    let pre_mesh_1 = pre_mesh_on_z_axis.get_pre_mesh(index - NB_BLOCK_ON_CHUNK_SIDE);
    let pre_mesh_2 = pre_mesh_on_z_axis.get_pre_mesh(index);

    if !pre_mesh_1.will_be_rendered() || !pre_mesh_2.will_be_rendered() {
        return;
    }

    if pre_mesh_1.get_block() != pre_mesh_2.get_block() {
        return;
    }

    let pre_mesh_1_stretch = pre_mesh_1.get_stretch();
    let pre_mesh_2_stretch = pre_mesh_2.get_stretch();

    if pre_mesh_1_stretch.get_y() != pre_mesh_2_stretch.get_y() {
        return;
    }

    let new_block = pre_mesh_2.get_block();
    let new_stretch = Stretch::from_xyz(
        pre_mesh_1_stretch.get_x().wrapping_add(1),
        pre_mesh_1_stretch.get_y(),
        1,
    );

    let new_pre_mesh = BlockPreMesh::new(new_block, new_stretch);
    pre_mesh_on_z_axis.set_pre_mesh(index, new_pre_mesh);
    pre_mesh_on_z_axis.should_not_be_rendered(index - NB_BLOCK_ON_CHUNK_SIDE);
}

fn merge_faces_on_z_axis_along_y(pre_mesh_on_z_axis: &mut ChunkPreMeshOneDirection, index: usize) {
    let pre_mesh_1 = pre_mesh_on_z_axis.get_pre_mesh(index - CHUNK_SIZE);
    let pre_mesh_2 = pre_mesh_on_z_axis.get_pre_mesh(index);

    if !pre_mesh_1.will_be_rendered() || !pre_mesh_2.will_be_rendered() {
        return;
    }

    if pre_mesh_1.get_block() != pre_mesh_2.get_block() {
        return;
    }

    let pre_mesh_1_stretch = pre_mesh_1.get_stretch();
    let pre_mesh_2_stretch = pre_mesh_2.get_stretch();

    if pre_mesh_1_stretch.get_x() != pre_mesh_2_stretch.get_x() {
        return;
    }

    let new_block = pre_mesh_2.get_block();
    let new_stretch = Stretch::from_xyz(
        pre_mesh_1_stretch.get_x(),
        pre_mesh_1_stretch.get_y().wrapping_add(1),
        1,
    );
    let new_pre_mesh = BlockPreMesh::new(new_block, new_stretch);
    pre_mesh_on_z_axis.set_pre_mesh(index, new_pre_mesh);
    pre_mesh_on_z_axis.should_not_be_rendered(index - CHUNK_SIZE);
}

pub fn merge_faces_on_y_axis(
    positive_y_pre_mesh: &mut ChunkPreMeshOneDirection,
    negative_y_pre_mesh: &mut ChunkPreMeshOneDirection,
) {
    for i in NB_BLOCK_ON_CHUNK_SIDE..NB_BLOCKS_PER_CHUNK {
        merge_faces_on_y_axis_along_x(positive_y_pre_mesh, i);
        merge_faces_on_y_axis_along_x(negative_y_pre_mesh, i);
    }

    for i in 0..NB_BLOCKS_PER_CHUNK {
        if i % CHUNK_SIZE == 0 {
            continue;
        }

        merge_faces_on_y_axis_along_z(positive_y_pre_mesh, i);
        merge_faces_on_y_axis_along_z(negative_y_pre_mesh, i);
    }
}

fn merge_faces_on_y_axis_along_x(pre_mesh_on_y_axis: &mut ChunkPreMeshOneDirection, index: usize) {
    let pre_mesh_1 = pre_mesh_on_y_axis.get_pre_mesh(index - NB_BLOCK_ON_CHUNK_SIDE);
    let pre_mesh_2 = pre_mesh_on_y_axis.get_pre_mesh(index);

    if !pre_mesh_1.will_be_rendered() || !pre_mesh_2.will_be_rendered() {
        return;
    }

    if pre_mesh_1.get_block() != pre_mesh_2.get_block() {
        return;
    }

    let pre_mesh_1_stretch = pre_mesh_1.get_stretch();
    let pre_mesh_2_stretch = pre_mesh_2.get_stretch();

    if pre_mesh_1_stretch.get_z() != pre_mesh_2_stretch.get_z() {
        return;
    }

    let new_block = pre_mesh_2.get_block();
    let new_stretch = Stretch::from_xyz(
        pre_mesh_1_stretch.get_x().wrapping_add(1),
        1,
        pre_mesh_1_stretch.get_z(),
    );

    let new_pre_mesh = BlockPreMesh::new(new_block, new_stretch);
    pre_mesh_on_y_axis.set_pre_mesh(index, new_pre_mesh);
    pre_mesh_on_y_axis.should_not_be_rendered(index - NB_BLOCK_ON_CHUNK_SIDE);
}

fn merge_faces_on_y_axis_along_z(pre_mesh_on_y_axis: &mut ChunkPreMeshOneDirection, index: usize) {
    let pre_mesh_1 = pre_mesh_on_y_axis.get_pre_mesh(index - 1);
    let pre_mesh_2 = pre_mesh_on_y_axis.get_pre_mesh(index);

    if !pre_mesh_1.will_be_rendered() || !pre_mesh_2.will_be_rendered() {
        return;
    }

    if pre_mesh_1.get_block() != pre_mesh_2.get_block() {
        return;
    }

    let pre_mesh_1_stretch = pre_mesh_1.get_stretch();
    let pre_mesh_2_stretch = pre_mesh_2.get_stretch();

    if pre_mesh_1_stretch.get_x() != pre_mesh_2_stretch.get_x() {
        return;
    }

    let new_block = pre_mesh_2.get_block();
    let new_stretch = Stretch::from_xyz(
        pre_mesh_1_stretch.get_x(),
        1,
        pre_mesh_1_stretch.get_z().wrapping_add(1),
    );
    let new_pre_mesh = BlockPreMesh::new(new_block, new_stretch);
    pre_mesh_on_y_axis.set_pre_mesh(index, new_pre_mesh);
    pre_mesh_on_y_axis.should_not_be_rendered(index - 1);
}
