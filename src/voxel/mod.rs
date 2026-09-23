use bevy::{gltf::gltf_ext::material, prelude::*};
use bevy::ecs::system::SystemParam;
use avian3d::prelude::*;

use std::collections::HashMap;

mod block;
mod chunk;
mod meshing;
mod tarrain;
mod components;
mod spawning;

use chunk::*;
use meshing::*;
use tarrain::*;
use spawning::*;
use components::*;

use crate::AppState;

pub struct VoxxelPlugin;

impl Plugin for VoxxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_chunk);
        app.init_resource::<ChunkMap>();
        //app.add_systems(OnEnter(AppState::InGame), (spawn_chunks_around_player).run_if(in_state(AppState::InGame)));
    }
}

// Wird vor allem geladen 
#[derive(Resource)]
pub struct ChunkMaterial(pub Handle<StandardMaterial>);

#[derive(SystemParam)]
pub struct ChunkParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub chunk_map: ResMut<'w, ChunkMap>,
    pub material: Res<'w, ChunkMaterial>,
}


// Basically ein chunk manager
#[derive(Resource, Default)]
pub struct ChunkMap(pub HashMap<IVec3, Entity>);
