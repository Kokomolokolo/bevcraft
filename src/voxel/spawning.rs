// Wird sich um das spawnen der chunks gekümmert

use std::time::Duration;

use bevy::prelude::*;
use avian3d::prelude::*;

use crate::{player::Player, voxel::{ChunkData, ChunkMap, ChunkParams, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos, meshing::build_chunk_mesh}, world::WorldGenerator};

const RENDER_DISTANCE: i32 = 20;
const WORLD_HEIGHT: i32 = 5;

pub fn spawn_chunks_around_player(mut spawner: ChunkParams, player_q: Query<&Transform, With<Player>>) { // Also erstmal nur so spawne
    use std::time::Instant;
    let now = Instant::now();

    // Spieler Position
    let Ok(player_transform) = player_q.single() else { return; };

    let player_chunk = IVec3::new(
        (player_transform.translation.x / CHUNK_SIZE as f32).floor() as i32, 
        0, 
        (player_transform.translation.z / CHUNK_SIZE as f32).floor() as i32
    );

    // Lazy chunk loading - TODO
    const MAX_CHUNKS_PER_FRAME: i32 = 20;
    let mut spawned_this_frame = 0;
    
    for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for z in -RENDER_DISTANCE..=RENDER_DISTANCE {

            for y in 0..WORLD_HEIGHT {
                let chunk_pos = IVec3::new(x, y, z) + player_chunk;
                let pos = ChunkPos(chunk_pos);
    
                // Wurde bereits gespawnt
                if spawner.chunk_map.0.contains_key(&pos) {
                    continue;
                }
                
                spawn_chunk(&mut spawner, chunk_pos);
                spawned_this_frame += 1;
    
                if spawned_this_frame >= MAX_CHUNKS_PER_FRAME {
                    return;
                }
            }
            
        }
    }
    let elapsed = now.elapsed();
    if elapsed > Duration::from_millis(0) {
        println!("Time to spawn chunks: {:.2?}", elapsed);
    }
}

pub fn generate_chunk_data_aroud_player(mut chunk_data: ResMut<ChunkData>, player_q: Query<&Transform, With<Player>>, generator: Res<WorldGenerator>) {
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
            for y in 0..WORLD_HEIGHT {
                let chunk_pos = IVec3::new(x, y, z) + player_chunk;
                generate_chunk_data(chunk_pos, &mut chunk_data, &generator);
            }
        }
    }
    let elapsed = now.elapsed();
    if elapsed > Duration::from_millis(1) {
        //println!("Time to generate chunk data: {:.2?}", elapsed);
    }
}

fn generate_chunk_data(pos: IVec3, chunk_data: &mut ChunkData, generator: &Res<WorldGenerator>) {
    let pos = ChunkPos(pos);
    
    if chunk_data.0.contains_key(&pos) {
        return;
    }

    // Chunk wird erstellt
    let mut chunk = Chunk::new();

    // Chunk wird nach generationsregeln bearbeitet
    generator.test_tarrain(&mut chunk, pos);
    
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
    
    if chunk.is_empty() {
        // Leere Chunks brauchen kein Mesh
        spawner.chunk_map.0.insert(pos, None);
        return;
    }
    
    let mesh = build_chunk_mesh(&chunk, &pos, neighbor_data);

    // Mit collider braucht das bauen eines meshes mehr als 3x so lang. Deswegen erstmal raus
    // let collider = Collider::trimesh_from_mesh(&mesh).expect("Chunk Mesh konnte nicht gebaut werden!");
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

    spawner.chunk_map.0.insert(pos, Some(entity));
}

///===============================
/// Despawning
///===============================
// muss noch überarbeitet werden, u.a. chunks auf allen höhen despawnen
pub fn despawn_chunks(
    mut chunk_data: ResMut<ChunkData>, 
    mut chunk_map: ResMut<ChunkMap>, 
    mut commands: Commands, 
    player_q: Query<&Transform, With<Player>>
) {
    let Ok(player_transform) = player_q.single() else { return; };
    
    let player_chunk = IVec3::new(
        (player_transform.translation.x / CHUNK_SIZE as f32).floor() as i32,
        0, // Egal, da alle Chunks auf jeder Höhe despawnt werden
        (player_transform.translation.z / CHUNK_SIZE as f32).floor() as i32 
    );

    // Da sonst 2 mut zugriffe
    let mut to_remove: Vec<ChunkPos> = Vec::new();
    
    for (chunk_pos, entity) in chunk_map.0.iter() {
        if (player_chunk.x - chunk_pos.0.x).abs() > RENDER_DISTANCE + 5 
        || (player_chunk.z - chunk_pos.0.z).abs() > RENDER_DISTANCE + 5
        {
            if let Some(entity) = entity { // Wenn wirklich ein entity da ist
                // Kann sein das keins da ist wenn der chunk bspw. leer ist
                commands.entity(*entity).despawn();
            }
            to_remove.push(*chunk_pos);
        }
    }
    // Aus den Resourcen entfernen
    for pos in to_remove {
        chunk_map.0.remove(&pos);
        //chunk_data.0.remove(&pos);
    }
}