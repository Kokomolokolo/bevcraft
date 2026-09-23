use bevy::{input::mouse::MouseMotion, math::VectorSpace, prelude::*};
use avian3d::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::player::{Player, movement};

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &Transform), With<Player>>,
) {
    const SPEED: f32 = 3.0;

    for (mut velocity, transform) in &mut query {
        let forward = transform.forward();
        let right = transform.right();
        let mut dir = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) {dir += *forward}
        if keyboard.pressed(KeyCode::KeyS) {dir -= *forward}
        if keyboard.pressed(KeyCode::KeyA) {dir -= *right}
        if keyboard.pressed(KeyCode::KeyD) {dir += *right}

        if dir != Vec3::ZERO{
            dir = dir.normalize();
        }
        
        velocity.x = dir.x * SPEED;
        velocity.z = dir.z * SPEED;
    }
}

pub fn player_look(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut pitch: Local<f32>,
) {
    const SENSITIVITY: f32 = 0.002;
    const PITCH_LIMIT: f32 = 1.54;

    let mut delta = Vec2::ZERO;

    for motion in mouse_motion.read() {
        delta += motion.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let pitch_delta = -delta.y * SENSITIVITY;

    if let Ok(mut player_transform) = player_query.single_mut() {
        player_transform.rotate_y(-delta.x * SENSITIVITY);
    }

    if let Ok(mut cam_transform) = camera_query.single_mut() {
        let new_pitch = (*pitch + pitch_delta).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        let applied = new_pitch - *pitch;
        *pitch = new_pitch;
        cam_transform.rotate_local_x(applied);
    }
}

pub fn player_jump(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut LinearVelocity, &ShapeHits), With<Player>>) {
    const JUMP_SPEED: f32 = 5.0;
    
    for (mut velocity, hits) in query.iter_mut() {
        let grounded = hits.iter().any(|hit| hit.normal1.y > 0.7);
        if !grounded {
            //println!("ALARMM!")
        }
        if grounded && keyboard.just_pressed(KeyCode::Space) {velocity.y = JUMP_SPEED}
    }
}

pub fn lock_cursor_on_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        cursor_options.grab_mode = CursorGrabMode::Locked;
        cursor_options.visible = false;
    }
}