use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

use crate::{player::Player, voxel::ChunkMap};
#[derive(Component, Clone, Copy, Default)]
pub struct HudFps;
#[derive(Component, Clone, Copy, Default)]
pub struct HudChunks;
#[derive(Component, Clone, Copy, Default)]
pub struct HudCoords;



pub fn setup_hud(mut commands: Commands) {
    println!("Spawning HUD");
    commands.spawn((
        Node {
            position_type: PositionType::Absolute, 
            left: px(5), 
            top: px(5), 
            padding: UiRect::all(px(3)), 
            display: Display::Flex, 
            flex_direction: FlexDirection::Column, 
            row_gap: px(2),
            ..default()
        },
    ))
    .with_children(|parent|  {
        let font_size = 12.;
        parent.spawn((
            Text::new("FPS"),
            TextFont {
                font_size: FontSize::Px(font_size),
                ..default()
            },
            TextColor::WHITE,
            HudFps
        ));
        parent.spawn((
            Text::new("COORDS"),
            TextFont {
                font_size: FontSize::Px(font_size),
                ..default()
            },
            TextColor::WHITE,
            HudCoords
        ));
        parent.spawn((
            Text::new("CHUNKS"),
            TextFont {
                font_size: FontSize::Px(font_size),
                ..default()
            },
            TextColor::WHITE,
            HudChunks
        ));
    });
}

pub fn update_hud(
    mut texts: ParamSet<(
        Query<&mut Text, With<HudFps>>,
        Query<&mut Text, With<HudCoords>>,
        Query<&mut Text, With<HudChunks>>,
    )>,

    player_q: Query<&Transform, With<Player>>,
    chunk_map: Res<ChunkMap>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
        .unwrap_or(-0.1);

    for mut text in &mut texts.p0() {
        *text = Text::new(format!("FPS: {:.0}", fps));
    }
    if let Ok(player) = player_q.single() {
        for mut text in &mut texts.p1() {
            *text = Text::new(format!("{:.0?}", player.translation))
        }
    }
    for mut text in &mut texts.p2() {
        let num_chunks = chunk_map.0.len();
        *text = Text::new(format!("Chunks: {:.0?}", num_chunks))
    }
}