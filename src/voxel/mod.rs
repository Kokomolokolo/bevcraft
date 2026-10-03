use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use std::collections::HashMap;

pub mod block;
pub mod chunk;
mod meshing;
pub mod components;
mod spawning;
mod chunk_data;
mod texture;

use chunk::*;
use meshing::*;
use spawning::*;
use components::*;

use crate::AppState;

pub struct VoxxelPlugin;

impl Plugin for VoxxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>();
        app.init_resource::<ChunkData>();
        app.add_systems(Update, (generate_chunk_data_aroud_player, spawn_chunks_around_player, despawn_chunks).chain().run_if(in_state(AppState::InGame)));
    }
}
// Basically ein chunk manager
#[derive(Resource, Default)]
pub struct ChunkMap(pub HashMap<ChunkPos, Option<Entity>>); // Eine Option für leere Chunks => leere Meshes

// Speichert alle Chunk daten 
#[derive(Resource, Default)]
pub struct ChunkData(pub HashMap<ChunkPos, Chunk>);

// Wird vor allem geladen 
#[derive(Resource)]
pub struct ChunkMaterial{
    pub transparent: Handle<StandardMaterial>,
    pub opaque: Handle<StandardMaterial>
}

#[derive(SystemParam)]
pub struct ChunkParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub chunk_map: ResMut<'w, ChunkMap>,
    pub chunk_data: ResMut<'w, ChunkData>,
    pub material: Res<'w, ChunkMaterial>,
}

