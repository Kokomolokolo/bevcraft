use std::{collections::HashMap};

use bevy::prelude::*;

use crate::voxel::{ChunkData, chunk::Chunk, components::ChunkPos};
impl ChunkData {
    // Gibt den Chunk selbst sowie alle Nachbarn zurück. Chunk selbst hat eine ChunkPos von 0
    pub fn get_chunk_and_neighbors(&self, pos: ChunkPos) -> HashMap<ChunkPos, &Chunk> {
        let mut return_map = HashMap::new();

        let offsets = [
            (0, 0, 0), // Chunk selber
            (1, 0, 0), // Rechts
            (-1, 0, 0), // Links
            (0, 1, 0), // Oben
            (0, -1, 0), // Unten
            (0, 0, 1), // Vorne
            (0, 0, -1), // Hinten
        ];
        let coord = pos.to_tupel();
        
        for offset in offsets {
            let offset_pos = IVec3::new(
                offset.0 + coord.0,
                offset.1 + coord.1,
                offset.2 + coord.2,
            );
            if let Some(offset_chunk) = self.0.get(&ChunkPos(offset_pos)) {
                return_map.insert(ChunkPos(offset_pos), offset_chunk);
            }
        }

        return_map
    } 
}
