use bevy::prelude::*;

use crate::AppState;

mod pig;

use pig::*;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_pig);
        app.add_systems(Update, move_pig);
    }
}
