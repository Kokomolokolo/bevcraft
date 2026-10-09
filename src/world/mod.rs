use bevy::prelude::*;

use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::{voxel::block::BlockType, world::biomes::BiomeRegistry};
// Hier wird alles logische Gepspawnt. Meine idee ist eine große API, eine generator struct, was dann beim spawnen jedes Chunks aufgerufen wird
// und dann wird der Chunk nach den Regeln gemacht; Caves, Tarrain, Structures etc.


mod tarrain;
mod biomes;
mod structures;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::thread_rng();
        let seed = rng.gen_range(0..420);
        app.insert_resource(WorldGenerator::new(seed));
    }
}
#[derive(Resource)]
pub struct WorldGenerator {
    seed: u32, // seed der Welt
    tarrain_noise: Perlin,
    detail_noise: Perlin,
    biom_noise: Perlin,
    registry: BiomeRegistry,
}

impl Default for WorldGenerator {
    fn default() -> Self {
        Self {
            seed: 420,
            tarrain_noise: Perlin::new(420),
            detail_noise: Perlin::new(420 * 420),
            biom_noise: Perlin::new(42),
            registry: BiomeRegistry::new(),
        }
    }
}

impl WorldGenerator {
    fn new(seed: u32) -> Self {
        Self {
            seed,
            tarrain_noise: Perlin::new(seed),
            detail_noise: Perlin::new(seed + 1),
            biom_noise: Perlin::new(seed + 2),
            registry: BiomeRegistry::new(),
        }
    }
    // pub fn get_climate(&self, x: f32, z: f32) -> (f32, f32) {
    //     let scale = 0.05;
    //     let temp = self.tempreture_noise.get([x as f64 * scale, z as f64 * scale]) as f32;
    //     let hum = self.tempreture_noise.get([x as f64 * scale, z as f64 * scale]) as f32;
    //     (temp, hum)
    // }
}