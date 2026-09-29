use bevy::prelude::*;

pub struct LoaderPlugin;

mod preloader;

use preloader::*;

use crate::AppState;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_block_material);
        //app.init_resource::<BevcraftAssets>();
    }
}

// Speicher alle Assets, die einmal am anfang gelanden werden müssen.
#[derive(Resource, Default)]
pub struct BevcraftAssets {
    pub atlas: Handle<Image>,
}