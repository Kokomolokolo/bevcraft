use bevy::{prelude::*};
use avian3d::prelude::*;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

mod movement;
mod interactions;

use interactions::player_mine_place_block;

use movement::*;

use crate::{AppState, settings::GameSettings, voxel::chunk::CHUNK_SIZE};

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(OnEnter(AppState::InGame), spawn_player);
        // app.add_systems(Update, (player_movement, player_look, lock_cursor_on_click, player_jump).run_if(in_state(AppState::InGame)));
        app.add_systems(Update, (player_mine_place_block).run_if(in_state(AppState::InGame)));
    }
}

fn spawn_player(
    mut commands: Commands,
    settings: Res<GameSettings>
) {
    let render_dist = (settings.render_distance * CHUNK_SIZE as i32) as f32; // Für Fog

    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.8),
        LockedAxes::ROTATION_LOCKED,
        Transform::from_xyz(5.0, 200.0, 5.0),
        ShapeCaster::new(
            Collider::sphere(0.4),
            Vec3::new(0.0, -1.35, 0.0), // knapp oberhalb der tatsächlichen Fußsohle (Zentrum minus ~1.4)
            Quat::default(),
            Dir3::NEG_Y,
        )
        .with_max_distance(0.1), // nur noch der kleine Rest bis zum Boden
        Visibility::default(),
    ))
    .with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            //Transform::from_xyz(1.0, 1.0, 1.0),
            DistanceFog {
                color: Color::srgb(0.2, 0.2, 0.2),
                falloff: FogFalloff::Linear { start: render_dist - 25., end: render_dist + 20. },
                ..default()
            }
        ));
    });
}