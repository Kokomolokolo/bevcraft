use bevy::{prelude::*, tasks::Task};
use bevy::ecs::system::SystemParam;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub mod block;
pub mod chunk;
mod meshing;
pub mod components;
mod spawning;
mod chunk_data;
mod texture;
mod dirty_chunks;

use chunk::*;
use meshing::*;
use spawning::*;
use components::*;
use dirty_chunks::*;

use crate::AppState;

pub struct VoxxelPlugin;

impl Plugin for VoxxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>();
        app.init_resource::<ChunkData>();
        app.init_resource::<ChunkSpawnQueue>();
        app.init_resource::<DirtyChunks>();
        app.init_resource::<LastPlayerChunk>();
        
        app.add_systems(OnEnter(AppState::InGame), build_chunk_queue); 
        app.add_systems(Update, (process_terrain_queue, process_structure_queue, process_mesh_queue, queue_chunks_around_player).run_if(in_state(AppState::InGame)));
        app.add_systems(Update, (update_player_chunk, handle_spawn_task, despawn_chunks, drain_chunks, rebuild_dirty_chunks).chain().run_if(in_state(AppState::InGame)));
    }
}
// Basically ein chunk manager
#[derive(Resource, Default)]
pub struct ChunkMap(pub HashMap<ChunkPos, Option<Entity>>); // Eine Option für leere Chunks => leere Meshes

// Speichert alle Chunk daten 
#[derive(Resource, Default)]
pub struct ChunkData(pub HashMap<ChunkPos, Chunk>);

#[derive(Resource, Default)]
pub struct DirtyChunks(pub HashSet<ChunkPos>);

// Wird vor allem geladen 
#[derive(Resource)]
pub struct ChunkMaterial{
    pub transparent: Handle<StandardMaterial>,
    pub opaque: Handle<StandardMaterial>
}

#[derive(Component)]
pub struct ComputeMeshTask(pub Task<ChunkMeshResult>); // Für das asycrone meshen

#[derive(Resource, Default)]
pub struct LastPlayerChunk(pub ChunkPos);

#[derive(SystemParam)]
pub struct ChunkParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub chunk_map: ResMut<'w, ChunkMap>,
    pub chunk_data: ResMut<'w, ChunkData>,
    pub material: Res<'w, ChunkMaterial>,
}

