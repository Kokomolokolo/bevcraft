// Chunk struct, Size, get, set

use crate::voxel::block::BlockType;

pub const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    blocks: Vec<BlockType> // Länge Chunksizepow3
}

impl Chunk {
    pub fn new() -> Self {
        Self { blocks: vec![BlockType::default(); CHUNK_SIZE.pow(3)] }
    }
    pub fn index(x: usize, y: usize, z: usize) -> usize {
        x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE // Das ist logisch wowi
    }
    pub fn get(&self, x: usize, y: usize, z: usize) -> BlockType {
        self.blocks[Self::index(x, y, z)]
    }
    pub fn set(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        self.blocks[Self::index(x, y, z)] = block
    }
}