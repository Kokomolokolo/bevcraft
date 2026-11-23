use std::{collections::HashMap, prelude::rust_2018};

use avian3d::collision::collider::Collider;
use bevy::{asset::RenderAssetUsages, prelude::*};

use crate::voxel::{
    block::BlockType::{self, Water}, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos, texture::{calculate_uvs, get_atlas_cords},
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

pub struct ChunkMeshResult {
    pub pos: ChunkPos,
    pub opaque: Option<Mesh>,
    pub transparent: Option<Mesh>,
    pub collider: Option<Collider>
    // vielleicht collider?
}

#[derive(Clone, Copy)]
enum FaceDirection {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
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


pub fn build_chunk_mesh(chunk: &Chunk, chunk_pos: &ChunkPos, chunk_neighbor: &HashMap<ChunkPos, &Chunk>) -> ChunkMeshResult {
    // Buffer statt die Hashmap, spaart cup zeit
    let buffer = create_padded_buffer(chunk, chunk_pos, &chunk_neighbor);

    let mut opaque_data = ChunkMeshData::default();
    let mut transparent_data = ChunkMeshData::default();

    let quads = find_greedy_quads(&buffer);

    for (x, y, z, width, height, dir, block) in quads {
        let target_data = //&mut opaque_data;
        if block.is_transparent() { &mut transparent_data} else { &mut opaque_data };

        add_greedy_face(x, y, z, width, height, block, dir, target_data);
    }

    
    ChunkMeshResult { 
        pos: *chunk_pos,
        opaque: if opaque_data.is_empty() { None } else {Some(opaque_data.build())}, 
        transparent: if transparent_data.is_empty() { None } else { Some(transparent_data.build()) },
        collider: None, // Wird in spawning.rs geregelt
    }
}



fn find_greedy_quads(
    buffer: &PaddedBuffer,
) -> Vec<(usize, usize, usize, usize, usize, FaceDirection, BlockType)> {
    let mut quads = Vec::new();

    let directions = [
        FaceDirection::Top,
        FaceDirection::Bottom,
        FaceDirection::Right,
        FaceDirection::Left,
        FaceDirection::Front,
        FaceDirection::Back,
    ];
    
    for dir in directions {
        for slice in 0..CHUNK_SIZE {
            let mut mask = generate_face_mask(slice, buffer, dir);
    
            for u in 0..CHUNK_SIZE {
                for v in 0..CHUNK_SIZE {
                    let idx = u * CHUNK_SIZE + v;
    
                    let Some(block) = mask[idx] else {
                        continue;
                    };

                    // Rechteck bestimmen
                    let width = find_width(&mask, u, v);
                    let height = find_height(&mask, u, v, width);
                    
                    let (x, y, z) = match dir {
                            FaceDirection::Back | FaceDirection::Front => (u, v, slice),
                            FaceDirection::Bottom | FaceDirection::Top => (u, slice, v),
                            FaceDirection::Left | FaceDirection::Right => (slice, u, v)
                    };
    
    
                    // In quads hinzufügen
                    quads.push((x, y, z, width, height, dir, block));
    
                    clear_mask(&mut mask, u, v, width, height);
                } 
            }
        }
    }

    quads
}


/// ======================
// Meshing
/// ======================

fn add_greedy_face (
    x: usize,
    y: usize,
    z: usize,
    width: usize,
    height: usize,
    block_type: BlockType,
    dir: FaceDirection,
    data: &mut ChunkMeshData,
) {
    let block_height = if block_type == Water { 0.8 } else { 1.0 };
    
    match dir {
        FaceDirection::Top => {
            let x = x as f32;
            let y = y as f32;
            let z = z as f32;
        
            let height = height as f32;
            let width = width as f32;
        
            let base = data.vertices.len() as u32;
            // data.vertices.extend_from_slice(&[
            //     [x, y + height, z + 1.0],
            //     [x + 1.0, y + height, z + 1.0],
            //     [x + 1.0, y + height, z],
            //     [x, y + height, z],
            // ]);
            data.vertices.extend_from_slice(&[
                [x, y + 1.0, z + width],
                [x + height, y + 1.0, z + width],
                [x + height, y + 1.0, z],
                [x, y + 1.0, z],
            ]);
            data.normals.extend_from_slice(&[[0.0, 1.0, 0.0]; 4]);
            data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
            data.colors.extend_from_slice(&[[1.0; 4]; 4]); // Den Slice der farben mal 4 für jeden Vertex
            // Texturing
            let (atlas_x, atlas_y) = get_atlas_cords(block_type, "top");
            let uv_cords = calculate_uvs(atlas_x, atlas_y);
            data.uvs.extend_from_slice(&uv_cords);
        },
        FaceDirection::Bottom => {
            let x = x as f32;
            let y = y as f32;
            let z = z as f32;
        
            let height = height as f32;
            let width = width as f32;
        
            let base = data.vertices.len() as u32;
            data.vertices.extend_from_slice(&[
                [x, y, z],
                [x + height, y, z ],
                [x + height, y, z + width],
                [x, y, z + width],
            ]);
            data.normals.extend_from_slice(&[[0.0, -1.0, 0.0]; 4]);
            data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
            data.colors.extend_from_slice(&[[1.0; 4]; 4]); // Den Slice der farben mal 4 für jeden Vertex
            // Texturing
            let (atlas_x, atlas_y) = get_atlas_cords(block_type, "bottom");
            let uv_cords = calculate_uvs(atlas_x, atlas_y);
            data.uvs.extend_from_slice(&uv_cords);
        },
        FaceDirection::Back => {
            let x = x as f32;
            let y = y as f32;
            let z = z as f32;
        
            let height = height as f32;
            let width = width as f32;
        
            let base = data.vertices.len() as u32;
            data.vertices.extend_from_slice(&[
                [x, y + width, z],
                [x + height, y + width, z],
                [x + height, y, z],
                [x, y, z],
            ]);
            data.normals.extend_from_slice(&[[0.0, 0.0, -1.0]; 4]);
            data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
            data.colors.extend_from_slice(&[[1.0; 4]; 4]); // Den Slice der farben mal 4 für jeden Vertex
            // Texturing
            let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
            let uv_cords = calculate_uvs(atlas_x, atlas_y);
            data.uvs.extend_from_slice(&uv_cords);
        },
        FaceDirection::Front => {
            let x = x as f32;
            let y = y as f32;
            let z = z as f32;
        
            let height = height as f32;
            let width = width as f32;
        
            let base = data.vertices.len() as u32;
            data.vertices.extend_from_slice(&[
                [x, y, z + 1.0],
                [x + height, y, z + 1.0],
                [x + height, y + width, z + 1.0],
                [x, y + width, z + 1.0],
            ]);
            data.normals.extend_from_slice(&[[0.0, 0.0, 1.0]; 4]);
            data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
            data.colors.extend_from_slice(&[[1.0; 4]; 4]); // Den Slice der farben mal 4 für jeden Vertex
            // Texturing
            let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
            let uv_cords = calculate_uvs(atlas_x, atlas_y);
            data.uvs.extend_from_slice(&uv_cords);
        },
        FaceDirection::Left => {
            let x = x as f32;
            let y = y as f32;
            let z = z as f32;
        
            let height = height as f32;
            let width = width as f32;
        
            let base = data.vertices.len() as u32;
            data.vertices.extend_from_slice(&[
                [x, y, z + width],
                [x, y + height, z + width],
                [x, y + height, z],
                [x, y, z],
            ]);
            data.normals.extend_from_slice(&[[-1.0, 0.0, 0.0]; 4]);
            data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
            data.colors.extend_from_slice(&[[1.0; 4]; 4]); // Den Slice der farben mal 4 für jeden Vertex
            // Texturing
            let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
            let uv_cords = calculate_uvs(atlas_x, atlas_y);
            data.uvs.extend_from_slice(&uv_cords);
        },
        FaceDirection::Right => {
            let x = x as f32;
            let y = y as f32;
            let z = z as f32;
        
            let height = height as f32;
            let width = width as f32;
        
            let base = data.vertices.len() as u32;
            data.vertices.extend_from_slice(&[
                [x + 1.0, y, z],
                [x + 1.0, y + height, z],
                [x + 1.0, y + height, z + width],
                [x + 1.0, y, z + width],
            ]);
            data.normals.extend_from_slice(&[[-1.0, 0.0, 0.0]; 4]);
            data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
            data.colors.extend_from_slice(&[[1.0; 4]; 4]); // Den Slice der farben mal 4 für jeden Vertex
            // Texturing
            let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
            let uv_cords = calculate_uvs(atlas_x, atlas_y);
            data.uvs.extend_from_slice(&uv_cords);
        },
        _ => {}
    }
}


/// ======================
// Hilfsfunktionen
/// ======================

// Generiert eine Mask aus Blocktype und None für die generierung des Meshes
// Für TOP: Ebene Y, Maske geht über X und Z
// Für Right/Left: X, Maske über Y und Z
// Für Front/Back: Z, Maske über X, Y
fn generate_face_mask(
    slice: usize,
    buffer: &PaddedBuffer,
    dir: FaceDirection,
) -> Vec<Option<BlockType>> {
    let mut mask = Vec::new();

    // Hier noch was mchen
    let offset = neighbor_offset(dir);
    
    for u in 0..CHUNK_SIZE { // x
        for v in 0..CHUNK_SIZE { // z
            // Koordinaten der Ebene
            let (x, y, z) = match dir {
                FaceDirection::Back | FaceDirection::Front => (u, v, slice),
                FaceDirection::Bottom | FaceDirection::Top => (u, slice, v),
                FaceDirection::Left | FaceDirection::Right => (slice, u, v)
            };
            
            let px = x + 1;
            let py = y + 1;
            let pz = z + 1;


            let block = buffer[pad_idx(px, py, pz)];

            let neighbor = buffer[pad_idx(
                (px as i32 + offset.x) as usize,
                (py as i32 + offset.y) as usize,
                (pz as i32 + offset.z) as usize,
            )];

            let render_top = should_render_face(block, neighbor);

            if render_top {
                mask.push(Some(block));
            } else {
                mask.push(None);
            }
        }
    }

    mask
}

// Maxiamle width an einer Koordinate finden
fn find_width(
    mask: &Vec<Option<BlockType>>,
    x: usize,
    z: usize, 
) -> usize {
    let mut width = 1;

    let idx = x * CHUNK_SIZE + z;
    // Gibt es ein face
    let Some(block) = mask[idx] else {
        return 0;
    };
    
    while z + width < CHUNK_SIZE {
        let next_idx = x * CHUNK_SIZE + (z + width);

        if mask[next_idx] == Some(block) {
            width += 1;
        } else {
            break;
        }
    }

    width
}

fn find_height(
    mask: &Vec<Option<BlockType>>,
    x: usize,
    z: usize,
    width: usize
) -> usize {
    let idx = x * CHUNK_SIZE + z;

    let Some(block) = mask[idx] else {
        return 0;
    };

    let mut height = 1;

    'outer: while x + height < CHUNK_SIZE {

        // Die höhe nach unten durchlaufen
        for dz in 0..width {
            let next_idx = (x + height) * CHUNK_SIZE + (dz + z);

            if mask[next_idx] != Some(block) {
                break 'outer;
            }
        }
        // Die ganze Reihe passt
        height += 1;
    }

    height
}

fn clear_mask(
    mask: &mut Vec<Option<BlockType>>,
    x: usize,
    z: usize,
    width: usize,
    height: usize,
) {
    for dx in 0..height {
        for dz in 0..width {
            let index = (x + dx) * CHUNK_SIZE + (z + dz);
            mask[index] = None;
        }
    }
}

fn neighbor_offset(dir: FaceDirection) -> IVec3 {
    match dir {
        FaceDirection::Back => IVec3::NEG_Z,
        FaceDirection::Bottom => IVec3::NEG_Y,
        FaceDirection::Front => IVec3::Z,
        FaceDirection::Left => IVec3::NEG_X,
        FaceDirection::Right => IVec3::X,
        FaceDirection::Top => IVec3::Y,
    }
}

/// ==================================================================
/// HILFSFUNKTIONEN
/// ==================================================================

fn should_render_face(curr: BlockType, neighbor: BlockType) -> bool {
    if curr == BlockType::Air {
        return false;
    }
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

    let height = if block_type == Water { 0.8 } else { 1.0 };

    // TOP (+Y)
    if render_top {
        let base = data.vertices.len() as u32;
        data.vertices.extend_from_slice(&[
            [x, y + height, z + 1.0],
            [x + 1.0, y + height, z + 1.0],
            [x + 1.0, y + height, z],
            [x, y + height, z],
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
            [x + 1.0, y + height, z],
            [x + 1.0, y + height, z + 1.0],
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
            [x, y + height, z + 1.0],
            [x, y + height, z],
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
            [x + 1.0, y + height, z + 1.0],
            [x, y + height, z + 1.0],
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
            [x, y + height, z],
            [x + 1.0, y + height, z],
        ]);
        data.normals.extend_from_slice(&[[0.0, 0.0, -1.0]; 4]);
        data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        data.colors.extend_from_slice(&[[0.7; 4]; 4]); // Leicht dunkler für bessere Kontur
        // texturing
        let (atlas_x, atlas_y) = get_atlas_cords(block_type, "side");
        let uv_cords = calculate_uvs(atlas_x, atlas_y);
        data.uvs.extend_from_slice(&uv_cords);
    }
}
