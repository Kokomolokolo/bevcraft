use bevy::{prelude::*};
use avian3d::prelude::*;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

mod movement;
mod interactions;
mod hotbar;

use interactions::player_mine_place_block;

use movement::*;

use crate::{AppState, settings::GameSettings, voxel::chunk::CHUNK_SIZE};

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player);
        app.insert_resource(Gravity(Vec3::NEG_Y * 15.));
        app.add_systems(Update, (player_movement, player_look, lock_cursor_on_click, player_jump).run_if(in_state(AppState::InGame)));
        app.add_systems(Update, (player_mine_place_block).run_if(in_state(AppState::InGame)));
    }
}

fn spawn_player(
    mut commands: Commands,
    settings: Res<GameSettings>
) {
    let render_dist = (settings.render_distance * CHUNK_SIZE as i32) as f32; // Für Fog
    if !settings.spawn_player {
        return;
    }
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Collider::capsule(0.5, 0.45),
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.0).with_combine_rule(CoefficientCombine::Min), // Verhindert das Haken an Wänden
        Transform::from_xyz(5.0, 80.0, 5.0),
        Visibility::default(),
    ))
    .with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 1.0, 0.0),
            DistanceFog {
                color: Color::srgb(0.2, 0.2, 0.2),
                falloff: FogFalloff::Linear { start: render_dist - 25., end: render_dist + 20. },
                ..default()
            }
        ));
    });
}