use bevy::prelude::*;

use crate::{voxel::block::BlockType, world::biomes::BiomeType::{Desert, Mountains, Ocean, Plains}};
#[derive(Debug)]
pub enum BiomeType {
    Plains,
    Desert,
    Ocean,
    Mountains
}

pub struct Biome {
    pub biome_type: BiomeType,
    pub level: f32, // Wo liegt es im Biom Noise, von -1 bis 1
    pub top_block: BlockType,
    pub base_height: f32,
    pub amplitude: f32, // Wie hoch sind Berge
    pub feature_frequency: f32, // Wie oft kommen berge
}

#[derive(Resource)]
pub struct BiomeRegistry {
    pub biomes: Vec<Biome>,
}
// Das muss besser gemacht werden!!
impl BiomeRegistry {
    pub fn new() -> Self {
        let mut biomes = Vec::new();
        biomes.push(PLAINS);
        biomes.push(DESERT);
        biomes.push(OCEAN);
        biomes.push(MOUTAINS);
        Self {
            biomes
        }
    }
}

pub const PLAINS: Biome = Biome {
    biome_type: Plains,
    level: 0.0,
    top_block: BlockType::Grass,
    base_height: 50.,
    amplitude: 20.,
    feature_frequency: 0.002,
};

pub const DESERT: Biome = Biome {
    biome_type: Desert,
    level: -0.4,
    top_block: BlockType::Sand,
    base_height: 50.,
    amplitude: 15.,
    feature_frequency: 0.002,
};

pub const OCEAN: Biome = Biome {
    biome_type: Ocean,
    level: -0.8,
    top_block: BlockType::Sand,
    base_height: 15., // Unter Sealevel
    amplitude: 4.,
    feature_frequency: 0.002,
};

pub const MOUTAINS: Biome = Biome {
    biome_type: Mountains,
    level: 0.8,
    top_block: BlockType::Stone,
    base_height: 55.,
    amplitude: 30.,
    feature_frequency: 0.07,
};

