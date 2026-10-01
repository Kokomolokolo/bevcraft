// Tarrain Via Noise

use bevy::{ecs::system::ResMut};
use noise::NoiseFn;

use crate::{voxel::{block::BlockType, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos}, world::{WorldGenerator, biomes::{self, Biome, BiomeRegistry, DESERT, MOUTAINS, OCEAN, PLAINS}}};

pub const SEA_LEVEL: i32 = 40;

impl WorldGenerator {
    pub fn generate_chunk_tarrain(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
        let chunk_offset = chunk_pos.to_world();
        
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk_offset.x + x as f32; // world y erst später berechnen. Erstmal nehme wir für die generierung den y wert nicht mit rein.
                let world_z = chunk_offset.z + z as f32;

                let biom_noise = self.biom_noise.get([world_x as f64 * 0.001, world_z as f64 * 0.001]);
                let (primary_biom, secondary_biom, t) = get_biomes(biom_noise as f32);

                let blended_base_height = lerp(primary_biom.base_height, secondary_biom.base_height, t);
                let blended_amplitude = lerp(primary_biom.amplitude, secondary_biom.amplitude, t);

                //let climate = self.get_climate(x as f32, z as f32);
                let base_noise = self.tarrain_noise.get([world_x as f64 * 0.01, world_z as f64 * 0.01]) as f32;
                let height = (blended_base_height + base_noise * blended_amplitude) as i32;
                
                for y in 0..CHUNK_SIZE {
                    let world_y = (chunk_offset.y + y as f32) as i32;
                    if world_y < SEA_LEVEL {
                        chunk.set(x, y, z, BlockType::Water);
                    }
                    else if world_y <= height {
                        if world_y == height {
                            chunk.set(x, y, z, primary_biom.top_block);
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

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn get_biomes(value: f32) -> (Biome, Biome, f32) {
    let ocean_cutoff = -0.7;
    let dessert_cutoff = -0.3;
    let plains_cutoff = 0.4;
    let mountains_cutoff = 1.0;
    
    if value < ocean_cutoff {
        (OCEAN, OCEAN, 0.0)
    } else if value < dessert_cutoff {
        let t = ((value - ocean_cutoff) / (dessert_cutoff - ocean_cutoff)).clamp(0.0, 1.0);
        (OCEAN, DESERT, t)
    } else if value < plains_cutoff {
        let t = ((value - dessert_cutoff) / (plains_cutoff - dessert_cutoff)).clamp(0.0, 1.0);
        (DESERT, PLAINS, t)
    } else {
        let t = ((value - plains_cutoff) / (mountains_cutoff - plains_cutoff)).clamp(0.0, 1.0);
        (PLAINS, MOUTAINS, t)
    }
}