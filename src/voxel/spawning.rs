// Wird sich um das spawnen der chunks gekümmert

use std::collections::{HashMap, VecDeque};

use bevy::{light::NotShadowCaster, prelude::*, tasks::{AsyncComputeTaskPool, futures_lite::future}};

use avian3d::prelude::*;

use crate::{player::Player, settings::GameSettings, voxel::{ChunkData, ChunkMap, ChunkParams, ComputeMeshTask, LastPlayerChunk, chunk::{CHUNK_SIZE, Chunk, GenState}, components::ChunkPos, meshing::{ChunkMeshResult, build_chunk_mesh}}, world::WorldGenerator};

pub const WORLD_HEIGHT: i32 = 5;

//======================
// Chunk Spawn Settings
//======================
pub const TARRAIN_CHUNKS_PER_FRAME: usize = 5;
pub const STRUCTURE_CHUNKS_PER_FRAME: usize = 3;
pub const MESH_TASKS_CHUNKS_PER_FRAME: usize = 5;
pub const MESH_UPLOADS_CHUNKS_PER_FRAME: usize = 5;

//======================
// Queing
//======================
#[derive(Resource, Default)]
pub struct ChunkSpawnQueue {
    // Alle Positonen der Tarrain Chunks die in die Pipeline kommen.
    // Neu Chunks, die neu generiert werden müssen fangen hier an und werden eingeseztzt
    // Warten auf Tarrain
    pub tarrain_queue: VecDeque<ChunkPos>,

    // Warten auf alle nachbarn und dann selbst auf structures.
    pub structure_queue: VecDeque<ChunkPos>,

    // Alle Positionen der Chunks, welche bereit zu spawnen sind.
    // Wartet darauf, dass alle nachbarn structuren haben
    pub meshing_queue: VecDeque<ChunkPos>,
}

impl ChunkSpawnQueue {
    fn add_to_queue(&mut self, pos: ChunkPos) {
        self.tarrain_queue.push_back(pos);
    }
    fn add_to_queue_priority(&mut self, pos: ChunkPos) {
        self.tarrain_queue.push_front(pos);
    }
}

pub fn build_chunk_queue(mut queue: ResMut<ChunkSpawnQueue>, settings: Res<GameSettings>) {
    // Schreibe alle Chunks zum Anfang in die Pipeline. 
    // Das wird seperat gemacht, da so zum Anfang die Chunks sortiert werden können.
    let render_dist = settings.render_distance + 2; // 2 mehr als gerendert werdne für culling und korekte Strukturen
    
    let mut positions = Vec::new();
    
    for x in -render_dist..=render_dist {
        for z in -render_dist..=render_dist {
            for y in 0..=WORLD_HEIGHT {
                positions.push(ChunkPos(IVec3::new(x, y, z)));
            }
        }
    }
    // Sortieren, sodass nähere Chunks schnelle Spawnen
    positions.sort_by_key(|pos| IVec2::new(pos.0.x, pos.0.z).length_squared());
    queue.tarrain_queue = VecDeque::from(positions);
    info!("Chunk queue initialized with {} chunks.", queue.tarrain_queue.len());
}

pub fn process_terrain_queue(
    mut queue: ResMut<ChunkSpawnQueue>,
    mut chunk_data: ResMut<ChunkData>,
    generator: Res<WorldGenerator>,
) {
    let limit = TARRAIN_CHUNKS_PER_FRAME;
    let mut processed = 0;

    println!("Terrain queue: {}, Structure queue: {}, Mesh queue: {}", queue.tarrain_queue.len(), queue.structure_queue.len(), queue.meshing_queue.len());
    
    while processed < limit {
        let Some(pos) = queue.tarrain_queue.pop_front() else {
            break; // Queue ist leer
        };

        // Wenn noch nicht da, Terrain generieren
        if !chunk_data.0.contains_key(&pos) {
            chunk_data.generate_chunk_tarrain_data(pos, &generator);
            
            if let Some(chunk) = chunk_data.0.get_mut(&pos) {
                chunk.state = GenState::Tarrain;
            } else {
                warn!("Unexpected: Chunk not found in chunk data even though it should be there")
            }

            // An die nächst Queue geben
            queue.structure_queue.push_back(pos);
            processed += 1;
        }
    }
}

pub fn process_structure_queue(
    mut queue: ResMut<ChunkSpawnQueue>,
    mut chunk_data: ResMut<ChunkData>,
    generator: Res<WorldGenerator>,
    settings: Res<GameSettings>,
    last_chunk: Res<LastPlayerChunk>, 
) {
    let limit = STRUCTURE_CHUNKS_PER_FRAME;
    let mut process = 0;
    let mut not_ready = Vec::new();
    let render_distance = settings.render_distance;
    let player_chunk = last_chunk.0.0;

    while let Some(pos) = queue.structure_queue.pop_front() {
        if process < limit {
            if chunk_data.has_all_neighbor_states(pos, GenState::Tarrain) {
                // Die structuren spawnen
                generator.spawn_structures(pos, &mut chunk_data);

                // State updaten
                if let Some(chunk) = chunk_data.0.get_mut(&pos) {
                    chunk.state = GenState::Structures;
                }

                let is_in_render_distance = 
                    (pos.0.x - player_chunk.x).abs() <= render_distance && 
                    (pos.0.z - player_chunk.z).abs() <= render_distance;                // Nur in die Meshing queue, wenn in der render distance.
                if is_in_render_distance {
                } 
                queue.meshing_queue.push_back(pos);
                process += 1;
                continue;
            }
        }
        // Entweder mehr als Limit oder nicht richtige Nachbarn
        not_ready.push(pos);
    }
    for pos in not_ready {
        queue.structure_queue.push_back(pos);
    }
}


pub fn process_mesh_queue(
    mut queue: ResMut<ChunkSpawnQueue>,
    // mut chunk_data: ResMut<ChunkData>,
    mut spawner: ChunkParams, 
) {
    let limit = MESH_TASKS_CHUNKS_PER_FRAME;
    let mut processed = 0;
    let mut not_ready = Vec::new();

    while processed < limit {
        let Some(pos) = queue.meshing_queue.pop_front() else {
            break; // Queue ist leer
        };

        // Wenn alle Nachbarchunks bereits Strukturen haben
        if spawner.chunk_data.has_all_neighbor_states(pos, GenState::Structures) {
            spawn_chunk_task(&mut spawner.commands, &mut spawner.chunk_map.0, &spawner.chunk_data, pos);

            if let Some(chunk) = spawner.chunk_data.0.get_mut(&pos) {
                chunk.state = GenState::Meshed;
            }

            processed += 1;
        }
        else {
            
            not_ready.push(pos);
        }
    }
    for pos in not_ready {
        queue.meshing_queue.push_back(pos);
    }
}

// ===============================
// Die Queue auffüllen
// ===============================
pub fn queue_chunks_around_player(
    mut spawner: ChunkParams, 
    player_q: Query<&Transform, With<Player>>, 
    settings: Res<GameSettings>,
    mut last_chunk: ResMut<LastPlayerChunk>,
    mut queue: ResMut<ChunkSpawnQueue>
) { // Also erstmal nur so spawne
    let render_distance = settings.render_distance as i32 + 2;
    
    // Spieler Position
    let Ok(player_transform) = player_q.single() else { return; };

    let player_chunk = IVec3::new(
        (player_transform.translation.x / CHUNK_SIZE as f32).floor() as i32, 
        0, 
        (player_transform.translation.z / CHUNK_SIZE as f32).floor() as i32
    );

    // Check ob der Spieler den Chunk wechselt. Wenn nein, dann wird nichts gespawnt
    if ChunkPos(player_chunk) == last_chunk.0 {
        // return;
    }
    
    for x in -render_distance..=render_distance {
        for z in -render_distance..=render_distance {

            for y in 0..=WORLD_HEIGHT {
                let chunk_pos = IVec3::new(x, y, z) + player_chunk;
                let pos = ChunkPos(chunk_pos);
    
                let exists_in_data = spawner.chunk_data.0.contains_key(&pos);
                let already_in_queue = queue.tarrain_queue.contains(&pos);

                // Nur hinzufügen, wenn er wirklich komplett neu ist und nicht wartet
                if !exists_in_data && !already_in_queue {
                    info!("Actually adding chunk {:?} to tarrain_queue", pos);
                    queue.tarrain_queue.push_back(pos);
                }
            }
        }
    }
}

// Spawner kann nicht so übergeben werden aufgrund von borrow checker Problemen
pub fn spawn_chunk_task(
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

    let Some(chunk) = *&neighbor_data.get(&pos) else {
        println!("Barely avoided a crash ;)");
        return;
    };

    // Check ob die Nachbarn alle dabei sind für korrektes cullling
    // Das sollte überflüssig sein, ich denke ich behalte es aber estmal
    if neighbor_data.len() < 7 {
        if pos.0.y == 0 || pos.0.y == WORLD_HEIGHT {
            if neighbor_data.len() != 6 {
                warn!("Wrong Chunk in queue");
                return;
            }
        } else {
            warn!("Wrong Chunk in queue");
            return;
        }
    }
    
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

        #[cfg(not(target_arch = "wasm32"))]
        let collider = mesh_result.opaque.as_ref().and_then(|m| {
            Collider::trimesh_from_mesh(m)
        });
        #[cfg(target_arch = "wasm32")]
        let collider = mesh_result.opaque.as_ref().and_then(|m| {
            None
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
    let current_tasks = tasks.iter().len();
    let uploads_per_frame = if current_tasks > 1000 {
        500
    } else if current_tasks > 500 {
        50
    } else {
    3
    };
    let mut uploads_this_frame = 0;
    
    for (entity, mut task) in tasks {
        if uploads_this_frame > uploads_per_frame {
            return;
        }
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
                    mesh.insert(RigidBody::Static);
                }

                let child_id = mesh.id();
                spawner.commands.entity(entity).add_child(child_id);
                uploads_this_frame += 1;
            }

            // Transparent
            if let Some(transparent) = result.transparent {
                let transparent_entity = spawner.commands.spawn((
                    Mesh3d(spawner.meshes.add(transparent)),
                    MeshMaterial3d(spawner.material.transparent.clone()),
                    Transform::IDENTITY,
                    // NotShadowCaster, // Bis ich das besser löse erstmal keine Schatten davon muss verbessert werden, das schaut sehr gut aus mit Leaves
                )).id();
                spawner.commands.entity(entity).add_child(transparent_entity);
                uploads_this_frame += 1;
            }
            // Die Task aus dem Entity entfernen
            spawner.commands.entity(entity).remove::<ComputeMeshTask>();
        }
    }
}

pub fn update_player_chunk(mut last_chunk: ResMut<LastPlayerChunk>, player_q: Query<&Transform, With<Player>>) {
    let Ok(player_transform) = player_q.single() else { return; };

    let player_chunk = IVec3::new(
        (player_transform.translation.x / CHUNK_SIZE as f32).floor() as i32, 
        0, 
        (player_transform.translation.z / CHUNK_SIZE as f32).floor() as i32
    );
    if ChunkPos(player_chunk) != last_chunk.0 {
        println!("Changed Chunk");
    }

    last_chunk.0 = ChunkPos(player_chunk);
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