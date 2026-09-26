// Wird sich um das spawnen der chunks gekümmert

use std::time::Duration;

use bevy::prelude::*;
use avian3d::prelude::*;

use crate::{player::Player, voxel::{ChunkData, ChunkParams, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos, meshing::build_chunk_mesh}, world::WorldGenerator};

const RENDER_DISTANCE: i32 = 20;

pub fn spawn_chunks_around_player(mut spawner: ChunkParams, player_q: Query<&Transform, With<Player>>) { // Also erstmal nur so spawne
    use std::time::Instant;
    let now = Instant::now();

    // Spieler Position
    let Ok(player_transform) = player_q.single() else { return; };

    let player_chunk = IVec3::new(
        (player_transform.translation.x / CHUNK_SIZE as f32) as i32, 
        0, 
        (player_transform.translation.z / CHUNK_SIZE as f32) as i32
    );

    // Lazy chunk loading - TODO
    const MAX_CHUNKS_PER_FRAME: i32 = 100;
    let mut spawned_this_frame = 0;
    
    for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
            let chunk_pos = IVec3::new(x, 0, z) + player_chunk;
            spawn_chunk(&mut spawner, chunk_pos);
            spawned_this_frame += 1;
        }
    }
    let elapsed = now.elapsed();
    if elapsed > Duration::from_millis(3) {
        println!("Time to spawn chunks: {:.2?}", elapsed);
    }
}

pub fn generate_chunk_data_aroud_player(mut chunk_data: ResMut<ChunkData>, player_q: Query<&Transform, With<Player>>) {
    // Debug Zeit messung
    use std::time::Instant;
    let now = Instant::now();

    // Spieler Position
    let Ok(player_transform) = player_q.single() else { return; };

    let player_chunk = IVec3::new(
        (player_transform.translation.x / CHUNK_SIZE as f32) as i32, 
        0, 
        (player_transform.translation.z / CHUNK_SIZE as f32) as i32
    );
    
    // Die Data wird mit einer höheren Distanze gerendert, damit inter chunk culling auch außen funktioniert
    const DATA_RENDER_DISTANCE: i32 = RENDER_DISTANCE + 2;
    for x in -DATA_RENDER_DISTANCE..=DATA_RENDER_DISTANCE {
        for z in -DATA_RENDER_DISTANCE..=DATA_RENDER_DISTANCE {
            let chunk_pos = IVec3::new(x, 0, z) + player_chunk;
            generate_chunk_data(chunk_pos, &mut chunk_data);
        }
    }
    let elapsed = now.elapsed();
    if elapsed > Duration::from_millis(1) {
        println!("Time to generate chunk data: {:.2?}", elapsed);
    }
}

fn generate_chunk_data(pos: IVec3, chunk_data: &mut ChunkData) {
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
    
    let chunk = neighbor_data.get(&pos).unwrap(); // Ob mich das nochmal abfuckt
    
    let mesh = build_chunk_mesh(&chunk, &pos, neighbor_data);
    //let collider = Collider::trimesh_from_mesh(&mesh).expect("Chunk Mesh konnte nicht gebaut werden!");
    let handle = spawner.meshes.add(mesh);

    // Chunk selbst spawnen
    let entity = spawner.commands.spawn((
        Mesh3d(handle),
        MeshMaterial3d(spawner.material.0.clone()),
        RigidBody::Static,
        //collider,
        Transform::from_translation(pos.to_world()),
        pos,
    )).id();

    spawner.chunk_map.0.insert(pos, entity);
}