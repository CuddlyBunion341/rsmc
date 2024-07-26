use crate::prelude::*;

pub fn update_debug_ui_system(debug_ui: Res<debug_ui_resources::DebugUi>) {}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
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
