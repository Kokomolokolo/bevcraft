use bevy::prelude::*;

// Hier wird alles logische Gepspawnt. Meine idee ist eine große API, eine generator struct, was dann beim spawnen jedes Chunks aufgerufen wird
// und dann wird der Chunk nach den Regeln gemacht; Caves, Tarrain, Structures etc.


mod tarrain;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        
    }
}
#[derive(Resource)]
pub struct WorldGenerator;
