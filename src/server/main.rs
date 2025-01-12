use bevy::prelude::*;

pub mod chat;
pub mod dashboard;
pub mod networking;
pub mod player;
pub mod prelude;
pub mod terrain;

use bevy_tui::{
    prelude::{initialize_terminal, teardown_terminal},
    MinimalTuiPlugins,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize_terminal()?;

    App::new()
        .add_plugins(MinimalTuiPlugins)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(networking::NetworkingPlugin)
        .add_plugins(terrain::TerrainPlugin)
        .add_plugins(chat::ChatPlugin)
        .add_plugins(dashboard::DashboardPlugin)
        .run();

    // teardown_terminal()?;

    Ok(())
}
