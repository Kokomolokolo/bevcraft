// Wird sich um das spawnen der chunks gekümmert

use std::collections::HashMap;

use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, futures_lite::future}};

use avian3d::prelude::*;

use crate::{player::Player, settings::GameSettings, voxel::{ChunkData, ChunkMap, ChunkParams, ComputeMeshTask, chunk::{CHUNK_SIZE, Chunk}, components::ChunkPos, meshing::{ChunkMeshResult, build_chunk_mesh}}, world::WorldGenerator};

pub const WORLD_HEIGHT: i32 = 5;

pub fn spawn_chunks_around_player(mut spawner: ChunkParams, player_q: Query<&Transform, With<Player>>, settings: Res<GameSettings>) { // Also erstmal nur so spawne

    let render_distance = settings.render_distance as i32;
    
    // Spieler Position
    let Ok(player_transform) = player_q.single() else { return; };

    let player_chunk = IVec3::new(
        (player_transform.translation.x / CHUNK_SIZE as f32).floor() as i32, 
        0, 
        (player_transform.translation.z / CHUNK_SIZE as f32).floor() as i32
    );

    // Lazy chunk loading - TODO
    const MAX_CHUNKS_PER_FRAME: i32 = 2000;
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
                
                spawn_chunk(&mut spawner.commands, &mut spawner.chunk_map.0, &spawner.chunk_data, pos);
                spawned_this_frame += 1;
    
                if spawned_this_frame >= MAX_CHUNKS_PER_FRAME {
                    return;
                }
            }
            
        }
    }
}

pub fn generate_chunk_data_aroud_player(
    mut chunk_data: ResMut<ChunkData>, 
    player_q: Query<&Transform, With<Player>>, 
    generator: Res<WorldGenerator>,
    settings: Res<GameSettings>,
) {
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
}

// Spawner kann nicht so übergeben werden aufgrund von borrow checker Problemen
pub fn spawn_chunk(
    commands: &mut Commands,
    chunk_map: &mut HashMap<ChunkPos, Option<Entity>>,
    chunk_data: &ChunkData,
    pos: ChunkPos
) {
    // Check ob an der Stelle bereits ein Chunk ist
    if chunk_map.contains_key(&pos) {
        return;
    }

    // Chunk sowie die neighbors werden geholt
    // Auch die Blöcke werden hier geholt
    let neighbor_data = chunk_data.get_chunk_and_neighbors(pos);

    let chunk = *neighbor_data.get(&pos).unwrap(); // Ob mich das nochmal abfuckt jaaaaaaaaaaa
    
    if chunk.is_empty() {
        // Leere Chunks brauchen kein Mesh
        chunk_map.insert(pos, None);
        return;
    }

    // Asyncrones meshing
    // Alles Kopieren für den neuen Thread
    let chunk_pos = pos; // Als eigener Wert wegen borrow checkers
    let owned_chunks: HashMap<ChunkPos, Chunk> = neighbor_data
        .into_iter()
        .map(|(chunk_pos, chunk)| (chunk_pos, chunk.clone()))
        .collect();

    // Task starten
    let thread_pool = AsyncComputeTaskPool::get();

    // Die task mit async move starten
    let task = thread_pool.spawn(async move {
        // Das läuft im Hintergrund
        let neighbors: HashMap<ChunkPos, &Chunk> = owned_chunks
            .iter()
            .map(|(pos, chunk)| (*pos, chunk))
            .collect();
    
        let chunk = owned_chunks
            .get(&chunk_pos)
            .expect("Chunk fehlt in owned_chunks");
        
        let mesh_result = build_chunk_mesh(chunk, &chunk_pos, &neighbors);

        let collider = mesh_result.opaque.as_ref().and_then(|m| {
            None
            //Collider::trimesh_from_mesh(m)
        });

        ChunkMeshResult {
            pos: chunk_pos,
            opaque: mesh_result.opaque,
            transparent: mesh_result.transparent,
            collider: collider
        }
    });
    
    // Leerer Parent-Container an Chunk-Position
    let parent_entity = commands.spawn((
        Transform::from_translation(pos.to_world()),
        Visibility::default(),
        ComputeMeshTask(task) // Mit der Compute Task
    )).id();
    
    chunk_map.insert(pos, Some(parent_entity));
}

pub fn handle_spawn_task(
    mut spawner: ChunkParams,
    mut tasks: Query<(Entity, &mut ComputeMeshTask)>
) {
    for (entity, mut task) in tasks {
        // Prüfen ob die Task fertig ist
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            // Fertig
            // Die beiden Meshes überprüfen und spawnen
            if result.opaque.is_none() && result.transparent.is_none() {
                // Wenn beide meshes leer sind kann das parent entity despawnt werden und die chunk map leer
                spawner.commands.entity(entity).despawn();
                spawner.chunk_map.0.insert(result.pos, None);
                continue;
            }

            // Sonst die beiden meshes spawnen
            if let Some(opaque) = result.opaque {
                let mut mesh = spawner.commands.spawn((
                    Mesh3d(spawner.meshes.add(opaque)),
                    MeshMaterial3d(spawner.material.opaque.clone()),
                    Transform::IDENTITY,
                ));

                // Der Collider, falls vorhanden
                if let Some(collider) = result.collider {
                    mesh.insert(collider);
                }

                let child_id = mesh.id();
                spawner.commands.entity(entity).add_child(child_id);
            }

            // Transparent
            if let Some(transparent) = result.transparent {
                let transparent_entity = spawner.commands.spawn((
                    Mesh3d(spawner.meshes.add(transparent)),
                    MeshMaterial3d(spawner.material.transparent.clone()),
                    Transform::IDENTITY,
                )).id();
                spawner.commands.entity(entity).add_child(transparent_entity);
            }
            // Die Task aus dem Entity entfernen
            spawner.commands.entity(entity).remove::<ComputeMeshTask>();
        }
    }
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