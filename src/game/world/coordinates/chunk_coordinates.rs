use bevy::prelude::*;

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
