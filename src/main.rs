use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use avian3d::prelude::*;
use bevy::light::CascadeShadowConfigBuilder;

mod voxel;
mod player;
mod assets;
mod camera;
mod world;
mod gui;
mod menu;
mod settings;
mod mobs;

use voxel::VoxxelPlugin;
use player::PlayerPlugin;
use assets::LoaderPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;
use gui::GUIPlugin;
use menu::MenuPlugin;
use settings::SettingsPlugin;
use mobs::MobPlugin;

use crate::settings::GameSettings;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    Settings,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true, // Passt das Canvas z.B. im Browser an
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()) // Wichtig für pixel-scharfe Texturen (Pixel-Art)
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins(CameraPlugin)
        .add_plugins((
            LoaderPlugin, 
            MenuPlugin, 
            SettingsPlugin, 
            VoxxelPlugin, 
            WorldPlugin, 
            PlayerPlugin, 
            GUIPlugin, 
            MobPlugin
        ))
        .add_plugins(PhysicsPlugins::default())
        // .add_plugins(PhysicsDebugPlugin::default())
        .run();
}

fn setup(
    mut commands: Commands, 
    settings: Res<GameSettings>,
) {
    if settings.shadows {
        commands.spawn((DirectionalLight {
                illuminance: 10000.0, // Helligkeit der Sonne
                shadow_maps_enabled: true,
                ..default()
            },
            CascadeShadowConfigBuilder {
                maximum_distance: 500.0, // Set max shadow distance
                first_cascade_far_bound: 20., // Set first cascade boundary
                num_cascades: 3,
                minimum_distance: 10.,
                ..default()
            }.build(),
            Transform::from_xyz(40.0, 80.0, 40.0)
                .looking_at(Vec3::ZERO, Vec3::Y)
        ));
    } else {
        commands.spawn((DirectionalLight {
                illuminance: 10000.0, // Helligkeit der Sonne
                ..default()
            },
            Transform::from_xyz(40.0, 80.0, 40.0)
                .looking_at(Vec3::ZERO, Vec3::Y)
        ));
    }
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.6, 0.99)));
}



// TODO
// Weltgeneration
// Texturen
// Meshing replacen statt immer despawnen
// Wolken
// Tiere
// Inventar
// Bessere Performance
// Spieler Blöcke abbauen, Springen, Springen, FOV Changes
// Main Menu
// World Saving
// Settings