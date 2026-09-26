use bevy::{math::VectorSpace, prelude::*};
use avian3d::prelude::*;

mod voxel;
mod mesh;
mod player;
mod assets;
mod camera;
mod world;

use voxel::VoxxelPlugin;
use player::PlayerPlugin;
use assets::LoaderPlugin;
use camera::CameraPlugin;
use voxel::ChunkMap;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    //Menu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins(CameraPlugin)
        .add_plugins((LoaderPlugin, VoxxelPlugin, PlayerPlugin))
        .add_plugins(PhysicsPlugins::default())
        //.add_plugins(PhysicsDebugPlugin::default())
        .run();
}

fn setup(
    mut commands: Commands, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        DirectionalLight::default(),
        AmbientLight::default(),
        Transform::from_xyz(100.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.6, 0.99)));
}