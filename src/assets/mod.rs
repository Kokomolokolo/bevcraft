use std::collections::HashMap;

use bevy::prelude::*;

pub struct LoaderPlugin;

mod preloader;

use preloader::*;


impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_assets, setup_block_material).chain());
    }
}

// Speicher alle Assets, die einmal am anfang gelanden werden müssen.
#[derive(Resource, Default)]
pub struct BevcraftAssets {
    pub models: HashMap<String, Handle<WorldAsset>>,
}