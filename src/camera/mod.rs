use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, (
            camera_look, 
            camera_movement, 
            lock_cursor_on_click,
            unlock_cursor_esc
        ));
    }
}

#[derive(Component)]
pub struct FpsCamera {
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for FpsCamera {
    fn default() -> Self {
        Self {
            speed: 50.0,
            sensitivity: 0.001,
        }
    }
}

pub fn setup_camera(
    mut commands: Commands,
    // Nutzt "Single" statt "Query", da es nur ein PrimaryWindow gibt
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false;

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        FpsCamera::default(),
    ));
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

fn unlock_cursor_esc(
    keys: Res<ButtonInput<KeyCode>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
    }
}

pub fn camera_movement(
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &FpsCamera)>,
) {
    for (mut transform, fps_cam) in &mut query {
        let mut velocity = Vec3::ZERO;

        let forward = transform.forward();
        let right = transform.right();

        if key.pressed(KeyCode::KeyW) {
            velocity += *forward;
        }
        if key.pressed(KeyCode::KeyS) {
            velocity -= *forward;
        }
        if key.pressed(KeyCode::KeyA) {
            velocity -= *right;
        }
        if key.pressed(KeyCode::KeyD) {
            velocity += *right;
        }

        if key.pressed(KeyCode::Space) {
            velocity.y += 1.0;
        }
        if key.pressed(KeyCode::ShiftLeft) {
            velocity.y -= 1.0;
        }

        transform.translation += velocity.normalize_or_zero() * fps_cam.speed * time.delta_secs();
    }
}

pub fn camera_look(
    // KORREKTUR: MessageReader anstelle von EventReader
    mut mouse_motion: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &FpsCamera)>,
) {
    for (mut transform, fps_cam) in &mut query {
        for motion in mouse_motion.read() {
            let yaw = -motion.delta.x * fps_cam.sensitivity;
            let pitch = -motion.delta.y * fps_cam.sensitivity;
            
            transform.rotate_y(yaw);
            transform.rotate_local_x(pitch);
        }
    }
}