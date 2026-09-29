// Tarrain Via Noise

use bevy::math::IVec3;
use noise::NoiseFn;

use crate::{voxel::{block::BlockType, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos}, world::WorldGenerator};

impl WorldGenerator {
    pub fn test_tarrain(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
        let chunk_offset = chunk_pos.to_world();
        
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk_offset.x + x as f32;
                let world_z = chunk_offset.z + z as f32;
                let noise_value = self.noise.get([world_x as f64 * 0.05, world_z as f64 * 0.05]) as f32;
                let height = ((noise_value * 10.) + 10.).max(2.0) as usize;

                for y in 0..CHUNK_SIZE {
                    if y <= height as usize {
                        if y == height {
                            chunk.set(x, y, z, BlockType::Grass);
                        } 
                        else if height > 5 {
                            chunk.set(x, y, z, BlockType::Stone);
                        }
                        else {
                            chunk.set(x, y, z, BlockType::Dirt);
                        }
                    }
                }
            }
        }
    }
}
