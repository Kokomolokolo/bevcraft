use bevy::prelude::*;

use crate::world::biom_stats::BiomStats;

#[derive(Resource, Default)]
pub struct BiomRegistry {
    pub biomes: Vec<BiomStats>
}

impl BiomRegistry {
    pub fn register(&mut self, biome: BiomStats) {
        self.biomes.push(biome);
    }

    pub fn get_dominant_biom(&self, tempreture: f32, humidity: f32) -> Option<&BiomStats> {
        self.biomes.iter().min_by(|a, b| {
            let dist_a = a.climat_distance(tempreture, humidity);
            let dist_b = b.climat_distance(tempreture, humidity);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}