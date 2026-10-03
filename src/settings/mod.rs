use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSettings::default());
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