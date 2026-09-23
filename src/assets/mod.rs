use bevy::prelude::*;

pub struct LoaderPlugin;

mod preloader;

use preloader::*;

use crate::AppState;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_chunk_material);
    }
}