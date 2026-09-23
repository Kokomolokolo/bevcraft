use bevy::{asset::RenderAssetUsages, prelude::*};

use crate::voxel::{
    block::BlockType,
    chunk::{Chunk, CHUNK_SIZE},
};

/// Baut das komplette Mesh für einen Chunk
pub fn build_chunk_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

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
                let render_top = !get_safe_block(chunk, x as i32, y as i32 + 1, z as i32).is_solid();
                let render_bottom = !get_safe_block(chunk, x as i32, y as i32 - 1, z as i32).is_solid();
                let render_right = !get_safe_block(chunk, x as i32 + 1, y as i32, z as i32).is_solid();
                let render_left = !get_safe_block(chunk, x as i32 - 1, y as i32, z as i32).is_solid();
                let render_back = !get_safe_block(chunk, x as i32, y as i32, z as i32 + 1).is_solid();
                let render_front = !get_safe_block(chunk, x as i32, y as i32, z as i32 - 1).is_solid();
                
                add_faces(
                    pos, 
                    block, 
                    &mut vertices, 
                    &mut normals, 
                    &mut indices, 
                    &mut uvs, 
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
}

fn get_safe_block(
    chunk: &Chunk,
    x: i32, 
    y: i32,
    z: i32
) -> BlockType {
    if x < 0 ||
        y < 0 ||
        z < 0 ||
        x >= CHUNK_SIZE as i32 ||
        y >= CHUNK_SIZE as i32 ||
        z >= CHUNK_SIZE as i32
        {
            return BlockType::Air;
        }
    return chunk.get(x as usize, y as usize, z as usize);
}

fn add_faces(
    pos: Vec3,
    block_type: BlockType, 
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    uvs: &mut Vec<[f32; 2]>,
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
    // TOP
    if render_top {
        let base = vertices.len() as u32;
        vertices.extend_from_slice(&[
            [pos.x as f32, pos.y as f32 + 1.0, pos.z as f32],
            [pos.x as f32 + 1.0, pos.y as f32 + 1.0, pos.z as f32],
            [pos.x as f32 + 1.0, pos.y as f32 + 1.0, pos.z as f32 + 1.0],
            [pos.x as f32, pos.y as f32 + 1.0, pos.z as f32 + 1.0],
        ]);

        normals.extend_from_slice(&[[0.0, 1.0, 0.0]; 4]);
        uvs.extend_from_slice(&[
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ]);
        indices.extend_from_slice(&[
            base,
            base + 1,
            base + 2,
            base,
            base + 2,
            base + 3,
        ]);
    }
    if render_bottom {
        let base = vertices.len() as u32;
        vertices.extend_from_slice(&[
            [x, y, z + 1.0],
            [x + 1.0, y, z + 1.0],
            [x + 1.0, y, z],
            [x, y, z],
        ]);

        normals.extend_from_slice(&[[0.0, -1.0, 0.0]; 4]);
        uvs.extend_from_slice(&[
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ]);
        indices.extend_from_slice(&[
            base, base + 1, base + 2,
            base, base + 2, base + 3,
        ]);
    }

    // RIGHT (+X)
    if render_right {
        let base = vertices.len() as u32;
        vertices.extend_from_slice(&[
            [x + 1.0, y, z],
            [x + 1.0, y, z + 1.0],
            [x + 1.0, y + 1.0, z + 1.0],
            [x + 1.0, y + 1.0, z],
        ]);

        normals.extend_from_slice(&[[1.0, 0.0, 0.0]; 4]);
        uvs.extend_from_slice(&[
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ]);
        indices.extend_from_slice(&[
            base, base + 1, base + 2,
            base, base + 2, base + 3,
        ]);
    }

    // LEFT (-X)
    if render_left {
        let base = vertices.len() as u32;
        vertices.extend_from_slice(&[
            [x, y, z + 1.0],
            [x, y, z],
            [x, y + 1.0, z],
            [x, y + 1.0, z + 1.0],
        ]);

        normals.extend_from_slice(&[[-1.0, 0.0, 0.0]; 4]);
        uvs.extend_from_slice(&[
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ]);
        indices.extend_from_slice(&[
            base, base + 1, base + 2,
            base, base + 2, base + 3,
        ]);
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
        uvs.extend_from_slice(&[
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ]);
        indices.extend_from_slice(&[
            base, base + 1, base + 2,
            base, base + 2, base + 3,
        ]);
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
        uvs.extend_from_slice(&[
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ]);
        indices.extend_from_slice(&[
            base, base + 1, base + 2,
            base, base + 2, base + 3,
        ]);
    }
}