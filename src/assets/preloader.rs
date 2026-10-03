use bevy::{image::ImageSampler, prelude::*};

use crate::{AppState, assets::BevcraftAssets, voxel::ChunkMaterial};

pub fn setup_block_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let atlas = asset_server.load("textures/texture_atlas.png");

    // Da Bevy standartmäßg Bilinear Filtering benutzt
    if let Some(mut image) = images.get_mut(&atlas) {
        image.sampler = ImageSampler::nearest();
        println!("Changes");
    }
    
    let opaque_handle = materials.add(StandardMaterial {
        base_color_texture: Some(atlas.clone()),
        perceptual_roughness: 0.7,
        metallic: 0.0,
        reflectance: 0.1,
        // base_color: Color::srgb(0.5, 0.5, 0.5),
        //cull_mode: None,
        //unlit: true,
        ..default()
    });
    let transparent_handle = materials.add(StandardMaterial {
        base_color_texture: Some(atlas),
        alpha_mode: AlphaMode::Mask(0.5),
        // base_color: Color::srgb(0.5, 0.5, 0.5),
        //cull_mode: None,
        //unlit: true,
        ..default()
    });
    commands.insert_resource(ChunkMaterial {opaque: opaque_handle, transparent: transparent_handle});
    println!("Next State");
    next_state.set(AppState::Menu);
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    
) {
    let atlas = asset_server.load("textures/texture_atlas.png");

    commands.insert_resource(BevcraftAssets {
        atlas
    });
    println!("Loaded atlas.")
}
