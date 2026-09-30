use bevy::prelude::*;

use crate::voxel::block::BlockType;

#[derive(Clone, Debug)]
pub struct BiomStats {
    pub name: &'static str,

    // Klimaziele, von 0-1
    pub target_teperature: f32,
    pub target_humidity: f32,

    // Tarrain Parameter
    pub base_height: f32, // Grundhöhte
    pub height_amplidutde: f32, // Wie hoch schlagen die Berge
    pub tarrain_noise_frequency: f32, // Wie oft gibt es Hügel
    pub detail_noise_frequency: f32, // Variation

    // Blöcke
    pub top_block: BlockType,

    // ggf. Strukturen
}

impl BiomStats {
    pub fn climat_distance(&self, tempreture: f32, humidity: f32) -> f32 {
        let dt = (self.target_teperature - tempreture).abs();
        let dh = (self.target_humidity - humidity).abs();
        (dt * dt + dh * dh).sqrt()
    }
}