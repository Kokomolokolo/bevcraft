// Wird sich um das spawnen der chunks gekümmert

use std::time::Duration;

use bevy::prelude::*;

use avian3d::prelude::*;

use crate::{player::Player, settings::GameSettings, voxel::{ChunkData, ChunkMap, ChunkParams, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos, meshing::build_chunk_mesh}, world::WorldGenerator};

const WORLD_HEIGHT: i32 = 5;

pub fn spawn_chunks_around_player(mut spawner: ChunkParams, player_q: Query<&Transform, With<Player>>, settings: Res<GameSettings>) { // Also erstmal nur so spawne
    use std::time::Instant;
    let now = Instant::now();

    let render_distance = settings.render_distance as i32;
    
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
    
    for x in -render_distance..=render_distance {
        for z in -render_distance..=render_distance {

            for y in 0..=WORLD_HEIGHT {
                let chunk_pos = IVec3::new(x, y, z) + player_chunk;
                let pos = ChunkPos(chunk_pos);
    
                // Wurde bereits gespawnt
                if spawner.chunk_map.0.contains_key(&pos) {
                    continue;
                }
                
                spawn_chunk(&mut spawner, pos);
                spawned_this_frame += 1;
    
                if spawned_this_frame >= MAX_CHUNKS_PER_FRAME {
                    return;
                }
            }
            
        }
    }
    let elapsed = now.elapsed();
    if elapsed > Duration::from_millis(3) {
        println!("Time to spawn chunks: {:.2?}", elapsed);
    }
}

pub fn generate_chunk_data_aroud_player(
    mut chunk_data: ResMut<ChunkData>, 
    player_q: Query<&Transform, With<Player>>, 
    generator: Res<WorldGenerator>,
    settings: Res<GameSettings>,
) {
    // Debug Zeit messung
    use std::time::Instant;
    let now = Instant::now();

    let render_distance = settings.render_distance as i32;

    // Spieler Position
    let Ok(player_transform) = player_q.single() else { return; };

    let player_chunk = IVec3::new(
        (player_transform.translation.x / CHUNK_SIZE as f32).floor() as i32, 
        0, 
        (player_transform.translation.z / CHUNK_SIZE as f32).floor() as i32
    );
    
    // Die Data wird mit einer höheren Distanze gerendert, damit inter chunk culling auch außen funktioniert
    let data_render_distance: i32 = render_distance + 2;
    for x in -data_render_distance..=data_render_distance {
        for z in -data_render_distance..=data_render_distance {
            for y in 0..=WORLD_HEIGHT {
                let chunk_pos = IVec3::new(x, y, z) + player_chunk;
                chunk_data.generate_chunk_tarrain_data(chunk_pos, &generator);
            }
        }
    }
    let elapsed = now.elapsed();
    if elapsed > Duration::from_millis(1) {
        //println!("Time to generate chunk data: {:.2?}", elapsed);
    }
}

pub fn spawn_chunk(spawner: &mut ChunkParams, pos: ChunkPos) {
    // Check ob an der Stelle bereits ein Chunk ist
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
    
    let mesh_result = build_chunk_mesh(&chunk, &pos, neighbor_data);

    let opaque_opt = mesh_result.opaque;
    let transparent_opt = mesh_result.transparent;
    
    if opaque_opt.is_none() && transparent_opt.is_none() {
        spawner.chunk_map.0.insert(pos, None);
        return;
    }
    
    // Leerer Parent-Container an Chunk-Position
    let parent_entity = spawner.commands.spawn((
        Transform::from_translation(pos.to_world()),
        Visibility::default(),
    )).id();
    
    if let Some(opaque_mesh) = opaque_opt {
        // Mit collider braucht das bauen eines meshes mehr als 2-3x so lang. Deswegen erstmal raus
        // Vielleicht eine Lösung wo nur die Chunks direkt um den Spieler einen Collider gebaut bekommen..?
        // Oder Custom Collider / Physiks Engine schreiben aber darauf gar keine Lust erstmal
        //let collider = Collider::trimesh_from_mesh(&opaque_mesh).expect("Chunk Mesh konnte nicht gebaut werden!");
        
        let opaque_child = spawner.commands.spawn((
            Mesh3d(spawner.meshes.add(opaque_mesh)),
            MeshMaterial3d(spawner.material.opaque.clone()),
            //collider,
            Transform::IDENTITY,
        )).id();
        spawner.commands.entity(parent_entity).add_child(opaque_child);
    }
    
    if let Some(transparent_mesh) = transparent_opt {
        let transparent_child = spawner.commands.spawn((
            Mesh3d(spawner.meshes.add(transparent_mesh)),
            MeshMaterial3d(spawner.material.transparent.clone()), // Transparentes Material!
            Transform::IDENTITY,
        )).id();
        spawner.commands.entity(parent_entity).add_child(transparent_child);
    }
    
    spawner.chunk_map.0.insert(pos, Some(parent_entity));
}

///===============================
/// Despawning
///===============================
// muss noch überarbeitet werden, u.a. chunks auf allen höhen despawnen
pub fn despawn_chunks(
    mut chunk_data: ResMut<ChunkData>, 
    mut chunk_map: ResMut<ChunkMap>, 
    mut commands: Commands, 
    player_q: Query<&Transform, With<Player>>,
    settings: Res<GameSettings>,
) {
    let Ok(player_transform) = player_q.single() else { return; };

    let render_distance = settings.render_distance as i32;
    
    let player_chunk = IVec3::new(
        (player_transform.translation.x / CHUNK_SIZE as f32).floor() as i32,
        0, // Egal, da alle Chunks auf jeder Höhe despawnt werden
        (player_transform.translation.z / CHUNK_SIZE as f32).floor() as i32 
    );

    // Da sonst 2 mut zugriffe
    let mut to_remove: Vec<ChunkPos> = Vec::new();
    
    for (chunk_pos, entity) in chunk_map.0.iter() {
        if (player_chunk.x - chunk_pos.0.x).abs() > render_distance + 5 
        || (player_chunk.z - chunk_pos.0.z).abs() > render_distance + 5
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

// Alle meshes entfernen
pub fn drain_chunks(keys: Res<ButtonInput<KeyCode>>, mut chunk_map: ResMut<ChunkMap>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::KeyP) {
        for (pos, entity) in chunk_map.0.clone() {
            if let Some(entity) = entity {
                commands.entity(entity).despawn();
            }
            chunk_map.0.remove(&pos);
        }
    }
}