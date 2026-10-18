use bevy::prelude::*;

use crate::{AppState, voxel::SpawningParameters};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSettings::default());
        app.add_systems(Update, change_render.run_if(in_state(AppState::InGame)));
    }
} 

#[derive(Resource)]
pub struct GameSettings {
    pub render_distance: i32,
    pub shadows: bool,
    pub spawn_player: bool,
    pub spawning_parameters: SpawningParameters,
}

impl SpawningParameters {
    fn native() -> Self {
        Self {
            tarrain_chunks_per_frame: 15,
            structure_chunks_per_frame: 3,
            mesh_tasks_per_frame: 7,
            mesh_uploads_per_frame: 4,
        }
    }
    fn wasm() -> Self {
        Self {
            tarrain_chunks_per_frame: 3,
            structure_chunks_per_frame: 1,
            mesh_tasks_per_frame: 2,
            mesh_uploads_per_frame: 1,
        }
    }
    fn ultra() -> Self {
        Self {
            tarrain_chunks_per_frame: 30,
            structure_chunks_per_frame: 10,
            mesh_tasks_per_frame: 20,
            mesh_uploads_per_frame: 10,
        }
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        if cfg!(target_arch = "wasm32") {
            Self {
                render_distance: 5,
                shadows: false,
                spawn_player: false,
                spawning_parameters: SpawningParameters::wasm(),
            }
        } else {
            Self {
                render_distance: 10,
                shadows: true,
                spawn_player: false,
                spawning_parameters: SpawningParameters::native(),
            }
        }
    }
}

fn change_render(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<GameSettings>) {
    if keys.just_pressed(KeyCode::ArrowUp) {
        settings.render_distance += 1;
        println!("{}", settings.render_distance)
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        settings.render_distance -= 1;
        println!("{}", settings.render_distance)
    }
}