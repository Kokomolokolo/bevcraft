// Chunk struct, Size, get, set

use crate::voxel::block::BlockType;

pub const CHUNK_SIZE: usize = 32;

#[derive(Clone)]
pub struct Chunk {
    blocks: Vec<BlockType>, // Länge Chunksizepow3
    pub state: GenState, // s. Implementierung

    // Zähler, wie weit der Chunk und seine Nachbarchunks sind
    // pub tarrain_neighbors: u8,
    pub structure_neighbors: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GenState { // Wegen Multipass generation
    None, // Beim Start auf nichs
    Tarrain, // Nur grobes Tarrain
    Structures, // Mit den Strukturen
    Meshed, // Fertig, mesh wurde/wird gebaut
}

impl Chunk {
    pub fn new() -> Self {
        Self { blocks: vec![BlockType::default(); CHUNK_SIZE.pow(3)], state: GenState::None, structure_neighbors: 0 }
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
    pub fn is_empty(&self) -> bool {
        self.blocks.iter().all(|b| *b == BlockType::Air)
    }
}