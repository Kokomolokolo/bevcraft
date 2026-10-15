use bevy::prelude::*;

use crate::{AppState, menu::{MenuMarker, button}, settings::GameSettings};

#[derive(Component, Clone, Copy, Default)]
pub struct SettingsMarkerSpawnAs;

pub fn settings_scene() -> impl Scene {
    bsn! {
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
        MenuMarker
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
                button("Back")
                on(|_event: On<Pointer<Press>>, mut next_state: ResMut<NextState<AppState>>|
                    next_state.set(AppState::Menu)
                )
            ),
            (
                SettingsMarkerSpawnAs
                button("Spawning as: Player")
                on(|_event: On<Pointer<Press>>, mut settings: ResMut<GameSettings>| {
                    if settings.spawn_player {
                        settings.spawn_player = false
                    } else {
                        settings.spawn_player = true;
                    }
                })
            ),
            (
                button("Toggle shadows")
                on(|_event: On<Pointer<Press>>, mut settings: ResMut<GameSettings>| {
                    settings.render_distance += 1;
                    println!("{}", settings.render_distance);
                })
            ),
        ]
    }
}

pub fn update_menu(spawn_as_q: Query<&mut Text, With<SettingsMarkerSpawnAs>>, mut settings: ResMut<GameSettings>) {
    for text in spawn_as_q {
        *text = Text::new(format!(settings.spawn_player));
    }
}

pub fn menu_check(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard.pressed(KeyCode::Escape) {
        next_state.set(AppState::InGame);
    }
}