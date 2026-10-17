use std::collections::HashMap;

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
        perceptual_roughness: 0.9,
        metallic: 0.0,
        reflectance: 0.1,
        // base_color: Color::srgb(0.5, 0.5, 0.5),
        //cull_mode: None,
        // unlit: true,
        ..default()
    });
    let transparent_handle = materials.add(StandardMaterial {
        base_color_texture: Some(atlas),
        perceptual_roughness: 0.9,
        metallic: 0.0,
        reflectance: 0.1,
        alpha_mode: AlphaMode::Mask(0.2),
        // cull_mode: None,
        // unlit: true,
        ..default()
    });
    commands.insert_resource(ChunkMaterial {opaque: opaque_handle, transparent: transparent_handle});
    println!("Next State");
    next_state.set(AppState::InGame);
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut models: HashMap<String, Handle<WorldAsset>> = HashMap::new();

    models.insert("pig".to_string(), asset_server.load(
        GltfAssetLabel::Scene(0).from_asset(format!("models/pig.glb"))
    ));
    commands.insert_resource(BevcraftAssets {
        models
    });
    println!("Loaded atlas.")
}
