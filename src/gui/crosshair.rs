use bevy::prelude::*;

pub fn spawn_crosshair(mut commands: Commands) {
    // Horitzontal
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: px(20),
            height: px(20),
            top: percent(50),
            left: percent(49),
            ..default()
        },
        children![
            (
                Node {
                    position_type: PositionType::Absolute,
                    width: px(20),
                    height: px(2),
                    top: px(9),
                    ..default()
                },
                BackgroundColor(Color::WHITE)
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    width: px(2),
                    height: px(20),
                    left: px(9),
                    ..default()
                },
                BackgroundColor(Color::WHITE)
            )
        ]
    ));
}