// Wird sich um das spawnen der chunks gekümmert

use bevy::prelude::*;
use avian3d::prelude::*;

use crate::{voxel::{ChunkData, ChunkParams, chunk::Chunk, components::ChunkPos, meshing::build_chunk_mesh}, world::WorldGenerator};

const RENDER_DISTANCE: i32 = 1;

pub fn spawn_chunks_around_player(mut spawner: ChunkParams) { // Also erstmal nur so spawne
    for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
            let coord = IVec3::new(x, 0, z);
            spawn_chunk(&mut spawner, coord);
        }
    }
}

pub fn generate_chunk_data_aroud_player(mut chunk_data: ResMut<ChunkData>) {
    // Debug Zeit messung
    use std::time::Instant;
    let now = Instant::now();
    
    // Die Data wird mit einer höheren Distanze gerendert, damit inter chunk culling auch außen funktioniert
    const DATA_RENDER_DISTANCE: i32 = RENDER_DISTANCE + 2;
    for x in -DATA_RENDER_DISTANCE..=DATA_RENDER_DISTANCE {
        for z in -DATA_RENDER_DISTANCE..=DATA_RENDER_DISTANCE {
            let coord = IVec3::new(x, 0, z);
            generate_chunk_data(coord, &mut chunk_data);
        }
    }
    let elapsed = now.elapsed();
    println!("Time to generate chunk data: {:.2?}", elapsed);
}

fn generate_chunk_data(pos: IVec3, mut chunk_data: &mut ChunkData) {
    let pos = ChunkPos(pos);
    
    if chunk_data.0.contains_key(&pos) {
        return;
    }

    // Chunk wird erstellt
    let mut chunk = Chunk::new();

    // Chunk wird nach generationsregeln bearbeitet
    WorldGenerator::test_tarrain(&mut chunk);
    
    // Daten werden gepeichert
    chunk_data.0.insert(pos, chunk);
}

pub fn spawn_chunk(spawner: &mut ChunkParams, coord: IVec3) {
    // Check ob an der Stelle bereits ein Chunk ist
    let pos = ChunkPos(coord);
    if spawner.chunk_map.0.contains_key(&pos) {
        return;
    }

    // Chunk sowie die neighbors werden geholt
    let neighbor_data = spawner.chunk_data.get_chunk_and_neighbors(pos);

    let mesh = build_chunk_mesh(&chunk);

    let collider = Collider::trimesh_from_mesh(&mesh).expect("Chunk Mesh konnte nicht gebaut werden!");
    let handle = spawner.meshes.add(mesh);

    // Chunk selbst spawnen
    let entity = spawner.commands.spawn((
        Mesh3d(handle),
        MeshMaterial3d(spawner.material.0.clone()),
        RigidBody::Static,
        collider,
        Transform::from_translation(pos.to_world()),
        pos,
    )).id();

    spawner.chunk_map.0.insert(pos, entity);
}