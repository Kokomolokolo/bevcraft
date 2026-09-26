use std::collections::HashMap;

use bevy::prelude::*;

use crate::voxel::chunk::CHUNK_SIZE;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos(pub IVec3);

impl ChunkPos {
    pub fn to_world(&self) -> Vec3 {
        Vec3::new(
            (self.0.x * CHUNK_SIZE as i32) as f32, 
            (self.0.y * CHUNK_SIZE as i32) as f32, 
            (self.0.z * CHUNK_SIZE as i32) as f32)
    }
    pub fn to_tupel(&self) -> (i32, i32, i32) {
        (self.0.x, self.0.y, self.0.z)
    }
}