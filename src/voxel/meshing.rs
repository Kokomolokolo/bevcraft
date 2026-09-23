use bevy::{asset::RenderAssetUsages, prelude::*};

use crate::voxel::chunk::{Chunk, CHUNK_SIZE};

pub fn build_chunk_mesh(chunk: &Chunk) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if !chunk.get(x, y, z).is_solid() {
                    continue;
                }

                let x = x as f32;
                let y = y as f32;
                let z = z as f32;

                // =========================================================
                // TOP (+Y)
                // =========================================================

                let base = positions.len() as u32;

                positions.extend_from_slice(&[
                    [x,     y + 1.0, z],
                    [x + 1.0, y + 1.0, z],
                    [x + 1.0, y + 1.0, z + 1.0],
                    [x,     y + 1.0, z + 1.0],
                ]);

                normals.extend_from_slice(&[
                    [0.0, 1.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 1.0, 0.0],
                ]);

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

                // =========================================================
                // BOTTOM (-Y)
                // =========================================================

                let base = positions.len() as u32;

                positions.extend_from_slice(&[
                    [x,     y, z],
                    [x,     y, z + 1.0],
                    [x + 1.0, y, z + 1.0],
                    [x + 1.0, y, z],
                ]);

                normals.extend_from_slice(&[
                    [0.0, -1.0, 0.0],
                    [0.0, -1.0, 0.0],
                    [0.0, -1.0, 0.0],
                    [0.0, -1.0, 0.0],
                ]);

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

                // =========================================================
                // FRONT (-Z)
                // =========================================================

                let base = positions.len() as u32;

                positions.extend_from_slice(&[
                    [x,     y,     z],
                    [x,     y + 1.0, z],
                    [x + 1.0, y + 1.0, z],
                    [x + 1.0, y,     z],
                ]);

                normals.extend_from_slice(&[
                    [0.0, 0.0, -1.0],
                    [0.0, 0.0, -1.0],
                    [0.0, 0.0, -1.0],
                    [0.0, 0.0, -1.0],
                ]);

                uvs.extend_from_slice(&[
                    [0.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                ]);

                indices.extend_from_slice(&[
                    base, base + 1, base + 2,
                    base, base + 2, base + 3,
                ]);

                // =========================================================
                // BACK (+Z)
                // =========================================================

                let base = positions.len() as u32;

                positions.extend_from_slice(&[
                    [x,     y,     z + 1.0],
                    [x + 1.0, y,     z + 1.0],
                    [x + 1.0, y + 1.0, z + 1.0],
                    [x,     y + 1.0, z + 1.0],
                ]);

                normals.extend_from_slice(&[
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0],
                ]);

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

                // =========================================================
                // LEFT (-X)
                // =========================================================

                let base = positions.len() as u32;

                positions.extend_from_slice(&[
                    [x, y,     z],
                    [x, y,     z + 1.0],
                    [x, y + 1.0, z + 1.0],
                    [x, y + 1.0, z],
                ]);

                normals.extend_from_slice(&[
                    [-1.0, 0.0, 0.0],
                    [-1.0, 0.0, 0.0],
                    [-1.0, 0.0, 0.0],
                    [-1.0, 0.0, 0.0],
                ]);

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

                // =========================================================
                // RIGHT (+X)
                // =========================================================

                let base = positions.len() as u32;

                positions.extend_from_slice(&[
                    [x + 1.0, y,     z],
                    [x + 1.0, y + 1.0, z],
                    [x + 1.0, y + 1.0, z + 1.0],
                    [x + 1.0, y,     z + 1.0],
                ]);

                normals.extend_from_slice(&[
                    [1.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                ]);

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
    }

    Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(bevy::mesh::Indices::U32(indices))
}