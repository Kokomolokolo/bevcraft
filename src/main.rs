use bevy::{math::VectorSpace, prelude::*};
use avian3d::prelude::*;

mod voxel;
mod mesh;
mod player;
mod assets;

use voxel::VoxxelPlugin;
use player::PlayerPlugin;
use assets::LoaderPlugin;
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
        PointLight {
            intensity: 10000000.,
            color: Color::WHITE,
            ..default()
        },
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}