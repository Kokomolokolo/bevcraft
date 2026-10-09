use bevy::prelude::*;

use crate::{voxel::{chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos}, world::{WorldGenerator}};

impl WorldGenerator {
    pub fn spawn_structures(&self, chunk: &mut Chunk, pos: ChunkPos) {
        let chunk_offset = pos.to_world();

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk_offset.x + x as f32;
                let world_z = chunk_offset.z + z as f32;

                if self.should_spawn_tree(world_x, world_z) {
                    // let surface_y = self.get_height(world_x, world_z)
                }
                
            }
        }
    }
    pub fn parse_scructure() {}
    fn should_spawn_tree(&self, world_x: f32, world_z: f32) -> bool {
        // Nur in Plains weiiß nicht wie ich das mache
        if world_x % 4. == 0. && world_z % 4. == 0. {
            return true;
        }
        false
    }
}