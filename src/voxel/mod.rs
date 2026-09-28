use bevy::{gltf::gltf_ext::material, prelude::*};
use bevy::ecs::system::SystemParam;
use avian3d::prelude::*;

use std::collections::HashMap;

pub mod block;
pub mod chunk;
mod meshing;
mod tarrain;
pub mod components;
mod spawning;
mod chunk_data;
mod texture;

use chunk::*;
use meshing::*;
use tarrain::*;
use spawning::*;
use components::*;

use crate::AppState;

pub struct VoxxelPlugin;

impl Plugin for VoxxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>();
        app.init_resource::<ChunkData>();
        app.add_systems(Update, (generate_chunk_data_aroud_player, spawn_chunks_around_player).run_if(in_state(AppState::InGame)));
    }
}
// Basically ein chunk manager
#[derive(Resource, Default)]
pub struct ChunkMap(pub HashMap<ChunkPos, Entity>);

// Speichert alle Chunk daten 
#[derive(Resource, Default)]
pub struct ChunkData(pub HashMap<ChunkPos, Chunk>);

// Wird vor allem geladen 
#[derive(Resource)]
pub struct ChunkMaterial(pub Handle<StandardMaterial>);

#[derive(SystemParam)]
pub struct ChunkParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub chunk_map: ResMut<'w, ChunkMap>,
    pub chunk_data: ResMut<'w, ChunkData>,
    pub material: Res<'w, ChunkMaterial>,
}

