// Tarrain Via Noise

use bevy::math::IVec3;
use noise::NoiseFn;

use crate::{voxel::{block::BlockType, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos}, world::WorldGenerator};

impl WorldGenerator {
    pub fn test_tarrain(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
        let chunk_offset = chunk_pos.to_world();
        
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk_offset.x + x as f32; // world y erst später berechnen. Erstmal nehme wir für die generierung den y wert nicht mit rein.
                let world_z = chunk_offset.z + z as f32;
                let climate = self.get_climate(x as f32, z as f32);
                let noise_value = self.tarrain_noise.get([world_x as f64 * 0.05, world_z as f64 * 0.05]) as f32;
                let height = ((noise_value * 20.) + 40.).max(0.1) as usize;
                
                for y in 0..CHUNK_SIZE {
                    let world_y = (chunk_offset.y + y as f32) as usize;
                    if world_y <= height as usize {
                        if world_y == height {
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
