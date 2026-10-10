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
        app.init_resource::<DirtyChunks>();
        app.add_systems(OnEnter(AppState::InGame), spawn_spawn_chunks); // Das bleibt hier nicht!
        app.add_systems(Update, (generate_chunk_data_aroud_player, spawn_chunks_around_player, handle_spawn_task, despawn_chunks, drain_chunks, rebuild_dirty_chunks).chain().run_if(in_state(AppState::InGame)));
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

#[derive(SystemParam)]
pub struct ChunkParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub chunk_map: ResMut<'w, ChunkMap>,
    pub chunk_data: ResMut<'w, ChunkData>,
    pub material: Res<'w, ChunkMaterial>,
}

