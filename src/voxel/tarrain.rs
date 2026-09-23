// Tarrain Via Noise

use crate::voxel::{block::BlockType, chunk::{CHUNK_SIZE, Chunk}};


pub fn test_tarrain(chunk: &mut Chunk) {
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                
                if y < 2 {
                    chunk.set(x, y, z, BlockType::Stone)
                }
                if x == 3 && z == 3 {
                    chunk.set(x, y, z, BlockType::Stone);
                }
                if x == 5 && z == 5 {
                    chunk.set(x, y, z, BlockType::Stone);
                }
                
            }
        }
    }
}