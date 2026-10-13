use bevy::{light::NotShadowCaster, prelude::*};

use crate::{settings::{self, GameSettings}, voxel::chunk::CHUNK_SIZE};

#[derive(Component)]
pub struct CloudTag;

pub fn spawn_clouds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
    settings: Res<GameSettings>,
) {
    for x in -settings.render_distance..=settings.render_distance {
        for z in -settings.render_distance..=settings.render_distance {
            if hash2d(x, z, 1) > 0.9 {
                commands.spawn((
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
                    MeshMaterial3d(material.add(StandardMaterial {
                        base_color: Color::srgba(0.8, 0.8, 0.8, 0.6),
                        alpha_mode: AlphaMode::Blend,
                        cull_mode: None,
                        ..default()
                    })),
                    NotShadowCaster,
                    Transform::from_translation(Vec3 { x: (x as f32) * CHUNK_SIZE as f32, y: 256.0, z: (z as f32) * CHUNK_SIZE as f32 }),
                    CloudTag,
                ));
            }
        }
    }
}

fn hash2d(x: i32, z: i32, seed: u32) -> f32 {
    let mut h = (x as u32)
        .wrapping_mul(374761393)
        .wrapping_add((z as u32).wrapping_mul(668265263))
        .wrapping_add(seed);
    h = (h ^ (h >> 13)).wrapping_mul(1274126161);
    (h ^ (h >> 16)) as f32 / u32::MAX as f32
}