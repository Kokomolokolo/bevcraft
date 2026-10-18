use bevy::prelude::*;

use crate::{AppState, menu::{MenuMarker, button}, settings::GameSettings};

#[derive(Component, Clone, Copy, Default)]
pub struct SettingsMarkerSpawnAs;

pub fn settings_scene() -> impl Scene {
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
        ImageNode {
            image: "background.png",
            image_mode: NodeImageMode::Stretch,
        }
        Children[
            (
                Text::new("Settings")
                TextFont {
                    font_size: FontSize::Px(50.)
                }
                TextShadow
                
            ),
            (
                button("Render Distance +")
                on(|_event: On<Pointer<Press>>, mut settings: ResMut<GameSettings>| {
                    settings.render_distance += 1;
                    println!("{}", settings.render_distance);
                })
            ),
            (
                button("Render Distance -")
                on(|_event: On<Pointer<Press>>, mut settings: ResMut<GameSettings>| {
                    
                    if settings.render_distance > 2 {
                        settings.render_distance -= 1;
                        println!("{}", settings.render_distance)
                    }
                })
            ),
            (
                button("Toggle shadows")
                on(|_event: On<Pointer<Press>>, mut settings: ResMut<GameSettings>| {
                    if settings.shadows {
                        settings.shadows = false
                    } else {
                        settings.shadows = true;
                    }
                })
            ),
            (
                SettingsMarkerSpawnAs
                button("Toggle Creative")
                on(|_event: On<Pointer<Press>>, mut settings: ResMut<GameSettings>| {
                    if settings.spawn_player {
                        settings.spawn_player = false
                    } else {
                        settings.spawn_player = true;
                    }
                })
            ),
            (
                button("Back")
                on(|_event: On<Pointer<Press>>, mut next_state: ResMut<NextState<AppState>>|
                    next_state.set(AppState::Menu)
                )
            ),
        ]
    }
}

pub fn update_menu(spawn_as_q: Query<&mut Text, With<SettingsMarkerSpawnAs>>, mut settings: ResMut<GameSettings>) {
    for mut text in spawn_as_q {
        *text = Text::new(format!("Test, {}", settings.spawn_player));
    }
}

pub fn settings_keybind(
    keyboard: Res<ButtonInput<KeyCode>>, 
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            AppState::InGame => next_state.set(AppState::Settings),
            AppState::Settings => next_state.set(AppState::InGame),
            _ => {}
        }
    }
}