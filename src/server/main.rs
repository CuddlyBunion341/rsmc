use bevy::prelude::*;

use bevy_tui::prelude::*;

use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;
use crate::prelude::*;

use bevy::app::AppExit;

pub mod chat;
pub mod dashboard;
pub mod networking;
pub mod player;
pub mod prelude;
pub mod terrain;

use bevy_log::LogPlugin;
use bevy_tui::{
    prelude::{initialize_terminal, teardown_terminal},
    MinimalTuiPlugins,
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_terminal()?;

    App::new()
        .add_plugins(MinimalTuiPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins(player::PlayerPlugin)
        .add_plugins(networking::NetworkingPlugin)
        .add_plugins(terrain::TerrainPlugin)
        .add_plugins(chat::ChatPlugin)
        .add_plugins(dashboard::DashboardPlugin)
        .run();

    teardown_terminal()?;

    Ok(())
}
