use bevy::prelude::*;

use crate::{AppState, gui::hud::*};

mod hud;
mod crosshair;

use crosshair::spawn_crosshair;

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(OnEnter(AppState::InGame), setup_hud);
        app.add_systems(OnEnter(AppState::InGame), (setup_hud, spawn_crosshair));

        app.add_systems(Update, (update_hud).run_if(in_state(AppState::InGame)));
    }
}


