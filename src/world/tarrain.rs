// Tarrain Via Noise

use bevy::prelude::*;

use noise::NoiseFn;

use crate::{voxel::{block::BlockType, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos}, world::{WorldGenerator, biomes::{Biome, BiomeRegistry}}};

pub const SEA_LEVEL: i32 = 40;

impl WorldGenerator {
    pub fn generate_chunk_tarrain(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
        let chunk_offset = chunk_pos.to_world();
        
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk_offset.x + x as f32; // world y erst später berechnen. Erstmal nehme wir für die generierung den y wert nicht mit rein.
                let world_z = chunk_offset.z + z as f32;

                let (height, biom) = self.get_height_and_biom(world_x as i32, world_z as i32);
                
                for y in 0..CHUNK_SIZE {
                    let world_y = (chunk_offset.y + y as f32) as i32;
                    if world_y < SEA_LEVEL {
                        chunk.set(x, y, z, BlockType::Water);
                    }
                    if world_y <= height {
                        if world_y == height {
                            chunk.set(x, y, z, biom.top_block);
                        } 
                        else if world_y > height - 5 {
                            chunk.set(x, y, z, biom.filler_block);
                        }
                        else {
                            chunk.set(x, y, z, BlockType::Stone);
                        }
                    }
                }
            }
        }
    }
    // HILFSFUNKTIONEN
    pub fn get_height_and_biom(&self, world_x: i32, world_z: i32) -> (i32, &Biome) {
        let biom_val = self.biom_noise.get([world_x as f64 * 0.005, world_z as f64 * 0.005]) as f32;
        let base_noise = self.tarrain_noise.get([world_x as f64 * 0.01, world_z as f64 * 0.01]) as f32;
        let detail_noise = self.tarrain_noise.get([world_x as f64 * 0.03, world_z as f64 * 0.03]) as f32;

        let (b1, b2, t) = self.registry.sample(biom_val as f32);

        let h1 = b1.calculate_height(base_noise, detail_noise);
        let h2 = b2.calculate_height(base_noise, detail_noise);

        let final_height = lerp(h1, h2, t);
        let primary_biom = if t < 0.5 { b1 } else { b1 }; 

        (final_height.round() as i32, primary_biom)
    }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
