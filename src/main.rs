use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use avian3d::prelude::*;

mod voxel;
mod player;
mod assets;
mod camera;
mod world;
mod gui;
mod menu;
mod settings;

use voxel::VoxxelPlugin;
use player::PlayerPlugin;
use assets::LoaderPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;
use gui::GUIPlugin;
use menu::MenuPlugin;
use settings::SettingsPlugin;

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
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // das könnte man mal aufräumen aber es geht irgendwie nicht so wirklich   
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins(CameraPlugin)
        .add_plugins((LoaderPlugin, MenuPlugin, SettingsPlugin, VoxxelPlugin, WorldPlugin, PlayerPlugin, GUIPlugin))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .run();
}

fn setup(
    mut commands: Commands, 
) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(100.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.6, 0.99)));
}



// TODO
// Weltgeneration
// Texturen
// Tiere
// Inventar
// Frustrum culling
// Bessere Performance
// Spieler Blöcke abbauen, Springen, Springen, FOV Changes
// Fog, 
// Main Menu
// World Saving
// Settings