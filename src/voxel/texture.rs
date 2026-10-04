use bevy::prelude::*;

use crate::voxel::block::BlockType;


pub fn get_atlas_cords(block_type: BlockType, face: &str) -> (usize, usize) {
    match (block_type, face) {
        (BlockType::Stone, _) => (0, 0),
        (BlockType::Grass, "top") => (3, 0),
        (BlockType::Grass, "side") => (2, 0),
        (BlockType::Grass, "bottom") => (1, 0),
        (BlockType::Dirt, _) => (1, 0),
        (BlockType::Sand, _) => (4, 0),
        (BlockType::Water, _) => (5, 0),
        (_, _) => (10, 10)
    }
}

pub fn calculate_uvs(atlas_x: usize, atlas_y: usize) -> [[f32; 2]; 4] {
    let atlas_size: f32 = 16.0;
    let uv_step = 1.0 / atlas_size;
    
    let padding = 0.5 / 256.0; // Die Atlas Pixel size

    let min_u = atlas_x as f32 * uv_step + padding; // Hilft bei Pixel bleeding
    let max_u = (atlas_x + 1) as f32 * uv_step - padding;

    let min_v = atlas_y as f32 * uv_step + padding;
    let max_v = (atlas_y + 1) as f32 * uv_step - padding;

    [ // Komentare sind falsch das ist mir alles suspekt
        [min_u, max_v], // 0: 
        [max_u, max_v], // 1: 
        [max_u, min_v], // 3: 
        [min_u, min_v], // 2: 
    ]
}