use avian3d::{collision::collider::Collider, dynamics::rigid_body::{LinearVelocity, LockedAxes, RigidBody}};
use bevy::prelude::*;

use crate::assets::BevcraftAssets;

#[derive(Component)]
pub struct Pig;

pub fn spawn_pig(
    mut commands: Commands,
    bevcraft_assets: Res<BevcraftAssets>
) {
    if let Some(pig_handle) = bevcraft_assets.models.get("pig") {
        commands.spawn((
            Pig,
            Visibility::default(),
            RigidBody::Static,
            LockedAxes::ROTATION_LOCKED,
            Collider::capsule(0.8, 0.4),
            Transform::from_xyz(0.0, 70.0, 0.0)
                .with_scale(Vec3::splat(0.05)),
        ))
        .with_children(|parent| {
            parent.spawn((
                WorldAssetRoot(pig_handle.clone()),
                Transform::from_xyz(0.0, 1.5, 0.0)
            ));
        });
    }
}

pub fn move_pig(pig_q: Query<&mut LinearVelocity, With<Pig>>) {
    for mut pig in pig_q {
        pig.x = 0.1;
    }
}