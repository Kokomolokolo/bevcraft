use std::collections::HashMap;

use bevy::{asset::RenderAssetUsages, prelude::*};

use crate::voxel::{
    block::BlockType, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos, texture::{calculate_uvs, get_atlas_cords},
};

/// Baut das komplette Mesh für einen Chunk
pub fn build_chunk_mesh(chunk: &Chunk, chunk_pos: &ChunkPos, chunk_neighbor: HashMap<ChunkPos, &Chunk>) -> Mesh {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let block = chunk.get(x, y, z);

                // Luft hat kein Mesh
                if !block.is_solid() {
                    continue;
                }
                let pos = Vec3::new(x as f32, y as f32, z as f32);

                // Wenn der Nachtbar nicht solid ist also Luft, Wasser etc.
                let render_top = !get_safe_block(chunk_pos, x as i32, y as i32 + 1, z as i32, &chunk_neighbor).is_solid();
                let render_bottom = !get_safe_block(chunk_pos, x as i32, y as i32 - 1, z as i32, &chunk_neighbor).is_solid();
                let render_right = !get_safe_block(chunk_pos, x as i32 + 1, y as i32, z as i32, &chunk_neighbor).is_solid();
                let render_left = !get_safe_block(chunk_pos, x as i32 - 1, y as i32, z as i32, &chunk_neighbor).is_solid();
                let render_back = !get_safe_block(chunk_pos, x as i32, y as i32, z as i32 - 1, &chunk_neighbor).is_solid();
                let render_front = !get_safe_block(chunk_pos, x as i32, y as i32, z as i32 + 1, &chunk_neighbor).is_solid();

                add_faces(
                    pos,
                    block,
                    &mut vertices,
                    &mut normals,
                    &mut indices,
                    &mut uvs,
                    &mut colors,
                    render_top,
                    render_bottom,
                    render_right,
                    render_left,
                    render_back,
                    render_front
                );
            }
        }
    }

    Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD
            | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        normals,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        uvs,
    )
    .with_inserted_indices(
        bevy::mesh::Indices::U32(indices)
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_COLOR, colors
    )
}

fn get_safe_block(
    chunk_pos: &ChunkPos,
    x: i32,
    y: i32,
    z: i32,
    neighbors: &HashMap<ChunkPos, &Chunk>,
) -> BlockType {
    let chunk_offset = IVec3::new(
        x.div_euclid(CHUNK_SIZE as i32),
        y.div_euclid(CHUNK_SIZE as i32),
        z.div_euclid(CHUNK_SIZE as i32)
    );

    let local_x = x.rem_euclid(CHUNK_SIZE as i32) as usize;
    let local_y = y.rem_euclid(CHUNK_SIZE as i32) as usize;
    let local_z = z.rem_euclid(CHUNK_SIZE as i32) as usize;

    let target_chunk_pos = ChunkPos(chunk_pos.0 + chunk_offset);

    match neighbors.get(&target_chunk_pos) {
        Some(chunk) => chunk.get(local_x, local_y, local_z),
        None => BlockType::Air // Zur Not immer Luft
    }
}

fn add_faces(
    pos: Vec3,
    block_type: BlockType,
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    uvs: &mut Vec<[f32; 2]>,
    colors: &mut Vec<[f32; 4]>,
    render_top: bool,
    render_bottom: bool,
    render_right: bool,
    render_left: bool,
    render_back: bool,
    render_front: bool,
) {
    let x = pos.x;
    let y = pos.y;
    let z = pos.z;

    // TOP (+Y)
    if render_top {
        let base = vertices.len() as u32;
        vertices.extend_from_slice(&[
            [x, y + 1.0, z + 1.0],
            [x + 1.0, y + 1.0, z + 1.0],
            [x + 1.0, y + 1.0, z],
            [x, y + 1.0, z],
        ]);
        normals.extend_from_slice(&[[0.0, 1.0, 0.0]; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        colors.extend_from_slice(&[[1.0; 4]; 4]); // Den Slice der farben mal 4 für jeden Vertex
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "top");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        uvs.extend_from_slice(&uv_cords);
    }

    // BOTTOM (-Y)
    if render_bottom {
        let base = vertices.len() as u32;
        vertices.extend_from_slice(&[
            [x, y, z],
            [x + 1.0, y, z],
            [x + 1.0, y, z + 1.0],
            [x, y, z + 1.0],
        ]);
        normals.extend_from_slice(&[[0.0, -1.0, 0.0]; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        colors.extend_from_slice(&[[0.6; 4]; 4]);
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "bottom");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        uvs.extend_from_slice(&uv_cords);
    }

    // RIGHT (+X)
    if render_right {
        let base = vertices.len() as u32;
        vertices.extend_from_slice(&[
            [x + 1.0, y, z + 1.0],
            [x + 1.0, y, z],
            [x + 1.0, y + 1.0, z],
            [x + 1.0, y + 1.0, z + 1.0],
        ]);
        normals.extend_from_slice(&[[1.0, 0.0, 0.0]; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        colors.extend_from_slice(&[[0.7; 4]; 4]);
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        uvs.extend_from_slice(&uv_cords);
    }

    // LEFT (-X)
    if render_left {
        let base = vertices.len() as u32;
        vertices.extend_from_slice(&[
            [x, y, z],
            [x, y, z + 1.0],
            [x, y + 1.0, z + 1.0],
            [x, y + 1.0, z],
        ]);
        normals.extend_from_slice(&[[-1.0, 0.0, 0.0]; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        colors.extend_from_slice(&[[0.7; 4]; 4]);
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        uvs.extend_from_slice(&uv_cords);
    }

    // FRONT (+Z)
    if render_front {
        let base = vertices.len() as u32;
        vertices.extend_from_slice(&[
            [x, y, z + 1.0],
            [x + 1.0, y, z + 1.0],
            [x + 1.0, y + 1.0, z + 1.0],
            [x, y + 1.0, z + 1.0],
        ]);
        normals.extend_from_slice(&[[0.0, 0.0, 1.0]; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        colors.extend_from_slice(&[[0.7; 4]; 4]);
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        uvs.extend_from_slice(&uv_cords);
    }

    // BACK (-Z)
    if render_back {
        let base = vertices.len() as u32;
        vertices.extend_from_slice(&[
            [x + 1.0, y, z],
            [x, y, z],
            [x, y + 1.0, z],
            [x + 1.0, y + 1.0, z],
        ]);
        normals.extend_from_slice(&[[0.0, 0.0, -1.0]; 4]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        colors.extend_from_slice(&[[0.6; 4]; 4]); // Leicht dunkler für bessere Kontur
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        uvs.extend_from_slice(&uv_cords);
    }
}
