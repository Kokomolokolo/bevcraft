use std::collections::HashMap;

use bevy::{asset::RenderAssetUsages, prelude::*};

use crate::voxel::{
    block::BlockType, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos, texture::{calculate_uvs, get_atlas_cords},
};
/// ==================================================================
/// Baut das komplette Mesh für einen Chunk
/// ==================================================================

pub fn build_chunk_mesh(chunk: &Chunk, chunk_pos: &ChunkPos, chunk_neighbor: HashMap<ChunkPos, &Chunk>) -> Mesh {
    // Buffer statt die Hashmap, spaart cup zeit
    let buffer = create_padded_buffer(chunk, chunk_pos, &chunk_neighbor);

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                
                let px = x + 1; // Da der Buffer bei -1 Beginnt
                let py = y + 1;
                let pz = z + 1;
                
                let block = chunk.get(x, y, z);

                // Luft hat kein Mesh
                if !block.is_solid() {
                    continue;
                }
                let pos = Vec3::new(x as f32, y as f32, z as f32);

                // Wenn der Nachtbar nicht solid ist also Luft, Wasser etc.
                let render_top = !buffer[pad_idx(px, py + 1, pz)].is_solid();
                let render_bottom = !buffer[pad_idx(px, py - 1, pz)].is_solid();
                let render_right = !buffer[pad_idx(px + 1, py, pz)].is_solid();
                let render_left = !buffer[pad_idx(px - 1, py, pz)].is_solid();
                let render_back = !buffer[pad_idx(px, py, pz - 1)].is_solid();
                let render_front = !buffer[pad_idx(px, py, pz + 1)].is_solid();

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
/// ==================================================================
/// HILFSFUNKTIONEN
/// ==================================================================

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
const PADDED_SIZE: usize = CHUNK_SIZE + 2;
pub type PaddedBuffer = [BlockType; PADDED_SIZE.pow(3)];

#[inline(always)] // Minimale Operation
pub fn pad_idx(x: usize, y: usize, z: usize) -> usize { // Erstellt aus 3 Koordinaten einen Index
    x + y * PADDED_SIZE + z * PADDED_SIZE * PADDED_SIZE
}
fn create_padded_buffer(
    chunk: &Chunk,
    chunk_pos: &ChunkPos,
    neighbors: &HashMap<ChunkPos, &Chunk>
) -> PaddedBuffer {
    let mut buffer = [BlockType::Air; PADDED_SIZE.pow(3)];

    for pad_x in 0..PADDED_SIZE {
        for pad_y in 0..PADDED_SIZE {
            for pad_z in 0..PADDED_SIZE {
                // Chunk Koordinaten
                let x = pad_x as i32 -1;
                let y = pad_y as i32 -1;
                let z = pad_z as i32 -1;

                let block: BlockType = if (0..CHUNK_SIZE as i32).contains(&x)
                    && (0..CHUNK_SIZE as i32).contains(&y)
                    && (0..CHUNK_SIZE as i32).contains(&z) {
                    chunk.get(x as usize, y as usize, z as usize)
                } else {
                    // An den Ränder aus den Nachbarchunk holen
                    get_safe_block(chunk_pos, x, y, z, neighbors)
                };

                buffer[pad_idx(pad_x, pad_y, pad_z)] = block;
            }
        }
    }
    buffer
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
