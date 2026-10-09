use bevy::prelude::*;

use crate::{voxel::block::BlockType, world::{biomes::BiomeType::{Desert, Mountains, Ocean, Plains, SpikyMountains}, tarrain::lerp}};
#[derive(Debug)]
pub enum BiomeType {
    Plains,
    Desert,
    Ocean,
    Mountains,
    SpikyMountains,
}

pub struct Biome {
    pub name: BiomeType,
    pub target_noise: f32, // Wo liegt es im Biom Noise, von -1 bis 1
    pub top_block: BlockType,
    pub filler_block: BlockType,
    pub base_height: f32,
    pub amplitude: f32, // Wie hoch sind Berge
    pub feature_frequency: f32, // Wie oft kommen berge
    pub ridge_factor: f32, // 0.0 weiche Hügel, 1.0 scharfte Spitzen
}

impl Biome {
    // Tarrainhöhe für das Biom
    pub fn calculate_height(&self, base_noise: f32, detail_noise: f32) -> f32 {
        let normal_height = base_noise;
        let ridged_height = (1.0 - detail_noise.abs()).powf(1.2) * 2.0 - 0.5;

        let blended_noise = lerp(normal_height, ridged_height, self.ridge_factor);

        self.base_height + blended_noise * self.amplitude
    }
}

pub struct BiomeRegistry {
    pub biomes: Vec<Biome>,
}
// Das muss besser gemacht werden!!
impl BiomeRegistry {
    pub fn new() -> Self {
        let mut biomes = vec![
            Biome {
                name: Plains,
                target_noise: 0.0,
                top_block: BlockType::Grass,
                filler_block: BlockType::Dirt,
                base_height: 50.,
                amplitude: 20.,
                feature_frequency: 0.002,
                ridge_factor: 0.1,
            },
            Biome {
                name: Ocean,
                target_noise: -0.8,
                top_block: BlockType::Sand,
                filler_block: BlockType::Stone,
                base_height: 15.,
                amplitude: 5.,
                feature_frequency: 0.002,
                ridge_factor: 0.0,
            },
            Biome {
                name: Desert,
                target_noise: -0.3,
                top_block: BlockType::Sand,
                filler_block: BlockType::Sand,
                base_height: 45.,
                amplitude: 10.,
                feature_frequency: 0.002,
                ridge_factor: 0.1,
            },
            Biome {
                name: Mountains,
                target_noise: 0.6,
                top_block: BlockType::Stone,
                filler_block: BlockType::Stone,
                base_height: 70.,
                amplitude: 45.,
                feature_frequency: 0.002,
                ridge_factor: 0.5,
            },
            Biome {
                name: SpikyMountains,
                target_noise: 0.9,
                top_block: BlockType::Stone, // Schnee?
                filler_block: BlockType::Stone,
                base_height: 85.,
                amplitude: 55.,
                feature_frequency: 0.002,
                ridge_factor: 0.8,
            },
        ];
        // Nach dem Target Noise sortieren
        biomes.sort_by(|a, b| a.target_noise.partial_cmp(&b.target_noise).unwrap());
        Self {
            biomes
        }
    }
    // Nächsten beiden Biome + t
    pub fn sample(&self, noise_val: f32) -> (&Biome, &Biome, f32) {
        let val = noise_val.clamp(-1.0, 1.0);

        // 1. Check für außerhalb des Spektrums
        if val <= self.biomes.first().unwrap().target_noise {
            return (&self.biomes[0], &self.biomes[0], 0.0);
        }
        if val >= self.biomes.last().unwrap().target_noise {
            let last = self.biomes.len() - 1;
            return (&self.biomes[last], &self.biomes[last], 0.0);
        }

        for i in 0..self.biomes.len() - 1 {
            let b1 = &self.biomes[i];
            let b2 = &self.biomes[i + 1];

            if val >= b1.target_noise && val <= b2.target_noise {
                let range = b2.target_noise - b1.target_noise;
                let raw_t = (val - b1.target_noise) / range;
                let t = smoothstep(raw_t); // S-Kurve
                return (b1, b2, t)
            }
        }
        (&self.biomes[0], &self.biomes[0], 0.0)
    }
}

pub fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}