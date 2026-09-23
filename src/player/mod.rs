use bevy::{prelude::*};
use avian3d::prelude::*;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

mod movement;
use movement::*;

use crate::AppState;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player);
        app.add_systems(Update, (player_movement, player_look, lock_cursor_on_click, player_jump).run_if(in_state(AppState::InGame)));
    }
}

fn spawn_player(
    mut commands: Commands
) {
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.8),
        LockedAxes::ROTATION_LOCKED,
        Transform::from_xyz(5.0, 14.0, 5.0),
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
        ));
    });
}