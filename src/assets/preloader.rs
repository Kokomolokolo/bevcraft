use bevy::{prelude::*};

use crate::{AppState, voxel::ChunkMaterial};

pub fn setup_chunk_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let handle = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            cull_mode: None,
            ..default()
        });
        commands.insert_resource(ChunkMaterial(handle));
    println!("Next State");
    next_state.set(AppState::InGame);
}