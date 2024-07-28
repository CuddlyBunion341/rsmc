use bevy::{
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
};

use crate::prelude::*;

pub fn update_debug_ui_system(debug_ui: Res<debug_ui_resources::DebugUi>) {}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraCode-Retina.ttf");

    let text_style = TextStyle {
        font_size: 60.0,
        color: Color::WHITE,
        font
    };

    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    });

    commands.spawn(Text2dBundle {
        text_anchor: Anchor::TopLeft,
        text: Text::from_section("Hello World!", text_style.clone())
            .with_justify(JustifyText::Left),
        ..default()
    });
}

// Player position:
// local position
// global position
// current chunk
//
// Terrain Data
// Number of Chunks
// Chunk Memory
// Height?
// Low?
//
// Server Data:
// Connected Clients Count
// Latency MS
