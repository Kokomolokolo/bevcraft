// Ähnlich wie spawning, kümmert sich um dirty chunks

use bevy::prelude::*;

use crate::voxel::{ChunkMap, ChunkParams, DirtyChunks, spawning::spawn_chunk};

pub fn rebuild_dirty_chunks(
    mut dirty_chunks: ResMut<DirtyChunks>,
    mut spawner: ChunkParams,
    //mut chunk_map: ResMut<ChunkMap> // Wird nur für das alte entity gebraucht. Das neue wird in spawn_chunks geschrieben
) {
    // löscht und spawnt alle 
    if dirty_chunks.0.is_empty() {
        return;
    }
    
    for pos in dirty_chunks.0.drain() {
        // Falls Entity da ist despawnen
        if let Some(Some(entity)) = spawner.chunk_map.0.remove(&pos) { // doppeltes some das ist doch nicht wahr
            spawner.commands.entity(entity).despawn();
        }

        // Neues entity
        spawn_chunk(&mut spawner, pos);
    }
}
