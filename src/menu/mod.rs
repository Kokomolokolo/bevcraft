use bevy::prelude::*;

mod main_menu;
mod settings_menu;

use main_menu::scene;
use settings_menu::settings_scene;
use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), (setup_camera, scene.spawn()).chain());
        app.add_systems(OnExit(AppState::Menu), despawn_menu);

        app.add_systems(OnEnter(AppState::Settings), (setup_camera, settings_scene.spawn()).chain());
        app.add_systems(OnExit(AppState::Settings), despawn_menu);


    }
}
#[derive(Component, Clone, Copy, Default)]
pub struct MenuMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MenuMarker));
}


fn despawn_menu(mut commands: Commands, menu_q: Query<Entity, With<MenuMarker>>) {
    for entity in menu_q {
        commands.entity(entity).despawn();
    }
}

fn button(lable: &str) -> impl Scene {
    bsn! {
        Button
        Node {
            width: px(350),
            height: px(50),
            border: px(5),
            border_radius: BorderRadius::MAX,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BorderColor::from(Color::BLACK)
        Children [(
            Text(lable)
            TextFont {
                font_size: px(33.0),
            }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
            TextShadow
        )]
    }
}
