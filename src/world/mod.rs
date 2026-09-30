use bevy::prelude::*;

use noise::{NoiseFn, Perlin};

use crate::{voxel::block::BlockType, world::{biom_registry::BiomRegistry, biom_stats::BiomStats}};
// Hier wird alles logische Gepspawnt. Meine idee ist eine große API, eine generator struct, was dann beim spawnen jedes Chunks aufgerufen wird
// und dann wird der Chunk nach den Regeln gemacht; Caves, Tarrain, Structures etc.


mod tarrain;
mod biom_registry;
mod biom_stats;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let mut registry = BiomRegistry::default();

        registry.register(BiomStats{
            name: "hills",
            target_humidity: 0.0,
            target_teperature: 0.2,
            base_height: 15.,
            height_amplidutde: 15.,
            tarrain_noise_frequency: 0.3,
            detail_noise_frequency: 0.03,
            top_block: BlockType::Grass
        });
        registry.register(BiomStats{
            name: "plains",
            target_humidity: -0.2,
            target_teperature: 0.2,
            base_height: 12.,
            height_amplidutde: 3.,
            tarrain_noise_frequency: 0.05,
            detail_noise_frequency: 0.03,
            top_block: BlockType::Grass
        });

        app.insert_resource(registry);
        app.insert_resource(WorldGenerator::new(420));
    }
}
#[derive(Resource)]
pub struct WorldGenerator {
    seed: u32, // seed der Welt
    tarrain_noise: Perlin,
    detail_noise: Perlin,
    humidity_noise: Perlin,
    tempreture_noise: Perlin,
    
}

impl Default for WorldGenerator {
    fn default() -> Self {
        Self {
            seed: 420,
            tarrain_noise: Perlin::new(420),
            detail_noise: Perlin::new(420 * 420),
            tempreture_noise: Perlin::new(42),
            humidity_noise: Perlin::new(4200)
        }
    }
}

impl WorldGenerator {
    fn new(seed: u32) -> Self {
        Self {
            seed,
            tarrain_noise: Perlin::new(seed),
            detail_noise: Perlin::new(seed + 1),
            tempreture_noise: Perlin::new(seed + 2),
            humidity_noise: Perlin::new(seed + 3)
        }
    }
    pub fn get_climate(&self, x: f32, z: f32) -> (f32, f32) {
        let scale = 0.05;
        let temp = self.tempreture_noise.get([x as f64 * scale, z as f64 * scale]) as f32;
        let hum = self.tempreture_noise.get([x as f64 * scale, z as f64 * scale]) as f32;
        (temp, hum)
    }
}