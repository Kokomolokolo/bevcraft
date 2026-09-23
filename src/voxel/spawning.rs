// Wird sich um das spawnen der chunks gekümmert

use bevy::prelude::*;
use avian3d::prelude::*;

use crate::voxel::{ChunkParams, chunk::Chunk, components::ChunkPos, meshing::build_chunk_mesh, tarrain::test_tarrain};

const RENDER_DISTANCE: i32 = 1;

pub fn spawn_chunks_around_player(mut spawner: ChunkParams) { // Also erstmal nur so spawne
    for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
            let coord = IVec3::new(x, 0, z);
            spawn_chunk(&mut spawner, coord);
        }
    }
}
pub fn spawn_test_chunk(spawner: &mut ChunkParams)  {
    spawn_chunk(spawner, IVec3 { x: 0, y: 0, z: 0 });
}


fn spawn_chunk(spawner: &mut ChunkParams, coord: IVec3) {
    // Check ob an der Stelle bereits ein Chunk ist
    if spawner.chunk_map.0.contains_key(&coord) {
        return;
    }
    let pos = ChunkPos(coord);
    let mut chunk = Chunk::new();

    // Hier erstmal auch tarrain generieren
    test_tarrain(&mut chunk);

    let mesh = build_chunk_mesh(&chunk);

    let collider = Collider::trimesh_from_mesh(&mesh).expect("Chunk Mesh konnte nicht gebaut werden!");
    let handle = spawner.meshes.add(mesh);

    // Chunk selbst spawnen
    let entity = spawner.commands.spawn((
        Mesh3d(handle),
        MeshMaterial3d(spawner.material.0.clone()),
        RigidBody::Static,
        collider,
        Transform::from_translation(pos.to_world()),
        pos,
    )).id();

    spawner.chunk_map.0.insert(coord, entity);
}