use bevy::prelude::*;

use crate::{voxel::{ChunkData, block::BlockType, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos}, world::{WorldGenerator, biomes::{Biome, BiomeType}, tarrain::SEA_LEVEL}};

pub struct StructureBlock {
    pub o_x: i32,
    pub o_y: i32,
    pub o_z: i32,
    pub block: BlockType,
}

pub struct Structure {
    pub blocks: Vec<StructureBlock>
}


impl WorldGenerator {
    pub fn spawn_structures(&self, pos: ChunkPos, chunk_data: &mut ChunkData) {
        let chunk_offset = pos.to_world();

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk_offset.x + x as f32;
                let world_z = chunk_offset.z + z as f32;

                let (surface, biom) = self.get_height_and_biom(world_x as i32, world_z as i32);

                if biom.name != BiomeType::Plains || surface < SEA_LEVEL {
                    continue;
                }

                let chance = hash2d(world_x as i32, world_z as i32, 2091);

                if chance < 0.02 {
                    // Baum spawnen
                    let tree = Structure::oak_tree();
                    self.place_structure(world_x as i32, surface, world_z as i32, chunk_offset, &tree, chunk_data);
                }
                
            }
        }
    }
    fn should_spawn_tree() {}
    fn place_structure(
        &self,
        world_origin_x: i32,
        world_origin_y: i32,
        world_origin_z: i32,
        chunk_offset: Vec3,
        structure: &Structure,
        // chunk: &mut Chunk, // Die meisten Blöcke sollten aufgrund von Performance im Chunk selbst platziert werden
        chunk_data: &mut ChunkData,
    ) {
        for structure_block in &structure.blocks {
            let wx = world_origin_x + structure_block.o_x;
            let wy = world_origin_y + structure_block.o_y;
            let wz = world_origin_z + structure_block.o_z;

            // Das hier ist ein wenig teuer also mal schauen
            chunk_data.set_world_block(wx, wy, wz, structure_block.block);
        }
    }
}


impl Structure {
    pub fn oak_tree() -> Self {
        let mut blocks = Vec::new();
        
        for y in 0..=7 {
            for x in -2..=2 {
                for z in -2..=2 {
                    if y == 7 {
                        if x == 0 && y == 0 {
                            blocks.push(StructureBlock {
                                o_x: x,
                                o_y: y,
                                o_z: z,
                                block: BlockType::OakLeave
                            });
                        }
                    }
                    if y == 5 || y == 6 {
                        if (x as i32).abs() < 2 && (z as i32).abs() < 2 {
                            blocks.push(StructureBlock {
                                o_x: x,
                                o_y: y,
                                o_z: z,
                                block: BlockType::OakLeave
                            });
                        }
                    }
                    if y == 3 || y == 4 {
                        blocks.push(StructureBlock {
                            o_x: x,
                            o_y: y,
                            o_z: z,
                            block: BlockType::OakLeave
                        });
                    }
                }
                
            }
            if y < 7 {
                blocks.push(StructureBlock {
                    o_x: 0,
                    o_y: y,
                    o_z: 0,
                    block: BlockType::OakLog
                });
            }
            
        }
        Self { blocks }
    }
}



/// Erzeugt eine deterministische Zufallszahl [0.0, 1.0] für ein (x, z) Koordinatenpaar
fn hash2d(x: i32, z: i32, seed: u32) -> f32 {
    let mut h = (x as u32)
        .wrapping_mul(374761393)
        .wrapping_add((z as u32).wrapping_mul(668265263))
        .wrapping_add(seed);
    h = (h ^ (h >> 13)).wrapping_mul(1274126161);
    (h ^ (h >> 16)) as f32 / u32::MAX as f32
}