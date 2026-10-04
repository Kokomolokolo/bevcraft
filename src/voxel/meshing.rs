use std::collections::HashMap;

use bevy::{asset::RenderAssetUsages, prelude::*};

use crate::voxel::{
    block::BlockType, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos, texture::{calculate_uvs, get_atlas_cords},
};
/// ==================================================================
/// Baut das komplette Mesh für einen Chunk
/// ==================================================================
#[derive(Default)]
pub struct ChunkMeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
    pub colors: Vec<[f32; 4]>,
}

impl ChunkMeshData {
    fn build(self) -> Mesh {
        Mesh::new(
            bevy::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD
                | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices,)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL,self.normals,)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0,self.uvs,)
        .with_inserted_indices(bevy::mesh::Indices::U32(self.indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, self.colors)
    }
    fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}

pub struct ChunkMeshResult {
    pub opaque: Option<Mesh>,
    pub transparent: Option<Mesh>,
}

pub fn build_chunk_mesh(chunk: &Chunk, chunk_pos: &ChunkPos, chunk_neighbor: HashMap<ChunkPos, &Chunk>) -> ChunkMeshResult {
    // Buffer statt die Hashmap, spaart cup zeit
    let buffer = create_padded_buffer(chunk, chunk_pos, &chunk_neighbor);

    let mut opaque_data = ChunkMeshData::default();
    let mut transparent_data = ChunkMeshData::default();

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
                let render_top = should_render_face(block, buffer[pad_idx(px, py + 1, pz)]);
                let render_bottom = should_render_face(block, buffer[pad_idx(px, py - 1, pz)]);
                let render_right = should_render_face(block, buffer[pad_idx(px + 1, py, pz)]);
                let render_left = should_render_face(block, buffer[pad_idx(px - 1, py, pz)]);
                let render_back = should_render_face(block, buffer[pad_idx(px, py, pz - 1)]);
                let render_front = should_render_face(block, buffer[pad_idx(px, py, pz + 1)]);

                // Auf welches Mesh soll hinzugefügt werden?
                let target_data = //&mut opaque_data;
                if block.is_transparent() { &mut transparent_data} else { &mut opaque_data };
                add_faces(
                    pos,
                    block,
                    target_data,
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
    ChunkMeshResult { 
        opaque: if opaque_data.is_empty() { None } else {Some(opaque_data.build())}, 
        transparent: if transparent_data.is_empty() { None } else { Some(transparent_data.build()) } }
}
/// ==================================================================
/// HILFSFUNKTIONEN
/// ==================================================================

fn should_render_face(curr: BlockType, neighbor: BlockType) -> bool {
    if neighbor == BlockType::Air {
        return true;
    }

    if curr.is_transparent() {
        !neighbor.is_solid() && neighbor != curr
    } else {
        !neighbor.is_solid() || neighbor.is_transparent()
    }
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
    data: &mut ChunkMeshData,
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
        let base = data.vertices.len() as u32;
        data.vertices.extend_from_slice(&[
            [x, y + 1.0, z + 1.0],
            [x + 1.0, y + 1.0, z + 1.0],
            [x + 1.0, y + 1.0, z],
            [x, y + 1.0, z],
        ]);
        data.normals.extend_from_slice(&[[0.0, 1.0, 0.0]; 4]);
        data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        data.colors.extend_from_slice(&[[1.0; 4]; 4]); // Den Slice der farben mal 4 für jeden Vertex
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "top");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        data.uvs.extend_from_slice(&uv_cords);
    }

    // BOTTOM (-Y)
    if render_bottom {
        let base = data.vertices.len() as u32;
        data.vertices.extend_from_slice(&[
            [x, y, z],
            [x + 1.0, y, z],
            [x + 1.0, y, z + 1.0],
            [x, y, z + 1.0],
        ]);
        data.normals.extend_from_slice(&[[0.0, -1.0, 0.0]; 4]);
        data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        data.colors.extend_from_slice(&[[0.6; 4]; 4]);
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "bottom");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        data.uvs.extend_from_slice(&uv_cords);
    }

    // RIGHT (+X)
    if render_right {
        let base = data.vertices.len() as u32;
        data.vertices.extend_from_slice(&[
            [x + 1.0, y, z + 1.0],
            [x + 1.0, y, z],
            [x + 1.0, y + 1.0, z],
            [x + 1.0, y + 1.0, z + 1.0],
        ]);
        data.normals.extend_from_slice(&[[1.0, 0.0, 0.0]; 4]);
        data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        data.colors.extend_from_slice(&[[0.7; 4]; 4]);
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        data.uvs.extend_from_slice(&uv_cords);
    }

    // LEFT (-X)
    if render_left {
        let base = data.vertices.len() as u32;
        data.vertices.extend_from_slice(&[
            [x, y, z],
            [x, y, z + 1.0],
            [x, y + 1.0, z + 1.0],
            [x, y + 1.0, z],
        ]);
        data.normals.extend_from_slice(&[[-1.0, 0.0, 0.0]; 4]);
        data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        data.colors.extend_from_slice(&[[0.7; 4]; 4]);
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        data.uvs.extend_from_slice(&uv_cords);
    }

    // FRONT (+Z)
    if render_front {
        let base = data.vertices.len() as u32;
        data.vertices.extend_from_slice(&[
            [x, y, z + 1.0],
            [x + 1.0, y, z + 1.0],
            [x + 1.0, y + 1.0, z + 1.0],
            [x, y + 1.0, z + 1.0],
        ]);
        data.normals.extend_from_slice(&[[0.0, 0.0, 1.0]; 4]);
        data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        data.colors.extend_from_slice(&[[0.7; 4]; 4]);
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        data.uvs.extend_from_slice(&uv_cords);
    }

    // BACK (-Z)
    if render_back {
        let base = data.vertices.len() as u32;
        data.vertices.extend_from_slice(&[
            [x + 1.0, y, z],
            [x, y, z],
            [x, y + 1.0, z],
            [x + 1.0, y + 1.0, z],
        ]);
        data.normals.extend_from_slice(&[[0.0, 0.0, -1.0]; 4]);
        data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        data.colors.extend_from_slice(&[[0.6; 4]; 4]); // Leicht dunkler für bessere Kontur
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        data.uvs.extend_from_slice(&uv_cords);
    }
}
