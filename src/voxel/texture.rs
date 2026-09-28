use bevy::prelude::*;

use crate::voxel::block::BlockType;


pub fn get_atlas_cords(block_type: BlockType, face: &str) -> (usize, usize) {
    match (block_type, face) {
        (BlockType::Stone, _) => (0, 0),
        (BlockType::Grass, "top") => (0, 4),
        (BlockType::Grass, "side") => (0, 3),
        (BlockType::Grass, "bottom") => (0, 2),
        (_, _) => (10, 10)
    }
}

pub fn calculate_uvs(atlas_x: usize, atlas_y: usize) -> [[f32; 2]; 4] {
    const atlas_size: f32 = 16.0;
    let uv_step = 1.0 / atlas_size;

    let min_u = atlas_x as f32 * uv_step;
    let max_u = (atlas_x + 1) as f32 * uv_step;

    let min_v = atlas_y as f32 * uv_step;
    let max_v = (atlas_y + 1) as f32 * uv_step;

    [
        [min_u, max_v], // 0: unten links
        [max_u, max_v], // 1: unten rechts
        [max_u, min_v], // 2: oben rechts
        [min_u, min_v], // 3: oben links
    ]
}