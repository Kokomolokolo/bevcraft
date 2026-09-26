use bevy::prelude::*;

use noise::Perlin;
// Hier wird alles logische Gepspawnt. Meine idee ist eine große API, eine generator struct, was dann beim spawnen jedes Chunks aufgerufen wird
// und dann wird der Chunk nach den Regeln gemacht; Caves, Tarrain, Structures etc.


mod tarrain;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldGenerator>();
    }
}
#[derive(Resource)]
pub struct WorldGenerator {
    seed: u32, // seed der Welt
    noise: Perlin,
}

impl Default for WorldGenerator {
    fn default() -> Self {
        Self {
            seed: 420,
            noise: Perlin::new(420)
        }
    }
}
