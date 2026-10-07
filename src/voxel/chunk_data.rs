use std::{collections::HashMap};

use bevy::prelude::*;

use crate::{voxel::{ChunkData, block::BlockType, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos}, world::WorldGenerator};

impl ChunkData {
    pub fn generate_chunk_tarrain_data(&mut self, pos: IVec3, generator: &Res<WorldGenerator>) {
        let pos = ChunkPos(pos);
        
        if self.0.contains_key(&pos) {
            return;
        }
    
        // Chunk wird erstellt
        let mut chunk = Chunk::new();
    
        // Chunk wird nach generationsregeln bearbeitet
        generator.generate_chunk_tarrain(&mut chunk, pos);
        
        // Daten werden gepeichert
        self.0.insert(pos, chunk);
    }

    pub fn set_world_block(&mut self, x: i32, y: i32, z: i32, block: BlockType) {
        let chunk_pos = ChunkPos(IVec3::new(
            x.div_euclid(CHUNK_SIZE as i32),
            y.div_euclid(CHUNK_SIZE as i32),
            z.div_euclid(CHUNK_SIZE as i32),
        ));

        if let Some(chunk) = self.0.get_mut(&chunk_pos) {
            let local_x = x.rem_euclid(CHUNK_SIZE as i32) as usize;
            let local_y = y.rem_euclid(CHUNK_SIZE as i32) as usize;
            let local_z = z.rem_euclid(CHUNK_SIZE as i32) as usize;

            chunk.set(local_x, local_y, local_z, block);
        }
    }
    pub fn get_world_block(&self, x: i32, y: i32, z: i32) -> Option<BlockType> {
        let chunk_pos = ChunkPos(IVec3::new(
            x.div_euclid(CHUNK_SIZE as i32),
            y.div_euclid(CHUNK_SIZE as i32),
            z.div_euclid(CHUNK_SIZE as i32),
        ));

        if let Some(chunk) = self.0.get(&chunk_pos) {
            let local_x = x.rem_euclid(CHUNK_SIZE as i32) as usize;
            let local_y = y.rem_euclid(CHUNK_SIZE as i32) as usize;
            let local_z = z.rem_euclid(CHUNK_SIZE as i32) as usize;

            return Some(chunk.get(local_x, local_y, local_z));
        }
        None
    }

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
