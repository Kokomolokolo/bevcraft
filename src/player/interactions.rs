use bevy::prelude::*;

use crate::{player::Player, voxel::{ChunkData, ChunkMap, DirtyChunks, block::BlockType, chunk::CHUNK_SIZE, components::ChunkPos}};

pub fn player_mine_place_block(
    camera_q : Query<(&Camera, &GlobalTransform)>,
    mut chunk_data: ResMut<ChunkData>,
    mut dirty_chunks: ResMut<DirtyChunks>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    // Raycasting
    // Blöcke zerstören
    if mouse_button.just_pressed(MouseButton::Left) {
        let ray_origin = camera_transform.translation();
        let ray_dir = camera_transform.forward();

        let max_dist = 7.;
        let step_size = 0.05;

        for i in 0..((max_dist / step_size) as i32) {
            let dist = i as f32 * step_size;

            let curr_pos = ray_origin + ray_dir * dist;

            let block_pos = IVec3::new(
                curr_pos.x.floor() as i32, 
                curr_pos.y.floor() as i32, 
                curr_pos.z.floor() as i32
            );

            // Option für einen Block
            let block_target = chunk_data.get_world_block(
                block_pos.x, block_pos.y, block_pos.z
            );

            match block_target {
                Some(BlockType::Air) => {
                    // Kann nicht abgebaut werden, weiter
                }
                Some(_block) => {
                    // In Chunkdata entfernen
                    println!("{}", block_pos);
                    
                    let chunk_pos = ChunkPos(IVec3::new(
                        block_pos.x.div_euclid(CHUNK_SIZE as i32),
                        block_pos.y.div_euclid(CHUNK_SIZE as i32),
                        block_pos.z.div_euclid(CHUNK_SIZE as i32),
                    ));
                    
                    chunk_data.set_world_block(block_pos.x, block_pos.y, block_pos.z, BlockType::Air);
                    dirty_chunks.0.insert(chunk_pos);
                    break;
                }
                None => {
                    return;
                }
            }
            
        }
    }

    // Platzieren
    if mouse_button.just_pressed(MouseButton::Right) {
        let ray_origin = camera_transform.translation();
        let ray_dir = camera_transform.forward();

        let max_dist = 9.;
        let step_size = 0.05;

        let mut last_air_block: Option<IVec3> = None;

        for i in 0..((max_dist / step_size) as i32) {
            let dist = i as f32 * step_size;

            let curr_pos = ray_origin + ray_dir * dist;

            let block_pos = IVec3::new(
                curr_pos.x.floor() as i32, 
                curr_pos.y.floor() as i32, 
                curr_pos.z.floor() as i32
            );

            // Option für einen Block
            let block_target = chunk_data.get_world_block(
                block_pos.x, block_pos.y, block_pos.z
            );

            match block_target {
                Some(BlockType::Air) => {
                    last_air_block = Some(block_pos);
                }
                Some(_block) => {
                    // Block gefunden, also am letzen Punkt wo Air war Block platzieren
                    println!("{}", block_pos);
                    if let Some(last_air_block) = last_air_block {
                        let chunk_pos = ChunkPos(IVec3::new(
                            last_air_block.x.div_euclid(CHUNK_SIZE as i32),
                            last_air_block.y.div_euclid(CHUNK_SIZE as i32),
                            last_air_block.z.div_euclid(CHUNK_SIZE as i32),
                        ));
                        
                        chunk_data.set_world_block(last_air_block.x, last_air_block.y, last_air_block.z, BlockType::Stone);
                        dirty_chunks.0.insert(chunk_pos);
                        break;
                    }
                }
                None => {
                    return;
                }
            }
            
        }
    }
}