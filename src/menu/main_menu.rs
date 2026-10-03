use bevy::prelude::*;

use crate::{AppState, menu::{MenuMarker, button}};

pub fn scene() -> impl Scene {
    bsn! {
        MenuMarker
        Node {
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            column_gap: px(5),
            row_gap: px(5),
        }
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2))
        Children[
            (
                Text::new("Bevcraft")
                TextFont {
                    font_size: FontSize::Vh(20.)
                }
                TextShadow
            ),
            (
                button("Join Game")
                on(|_event: On<Pointer<Press>>, mut next_state: ResMut<NextState<AppState>>| 
                    next_state.set(AppState::InGame)
                )
            ),
            (
                button("Settings")
                on(|_event: On<Pointer<Press>>, mut next_state: ResMut<NextState<AppState>>| 
                    next_state.set(AppState::Settings)
                )
            )
        ]
        
    }
}
