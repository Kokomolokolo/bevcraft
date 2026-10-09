use bevy::prelude::*;

use crate::AppState;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSettings::default());
        app.add_systems(Update, change_render.run_if(in_state(AppState::InGame)));
    }
} 

#[derive(Resource)]
pub struct GameSettings{
    pub render_distance: i32
}

impl Default for GameSettings {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        let render = 10;
        #[cfg(not(target_arch = "wasm32"))]
        let render = 20;

        Self {
            render_distance: render
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