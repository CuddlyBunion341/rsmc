pub mod chat;
pub mod dashboard;
pub mod networking;
pub mod player;
pub mod prelude;
pub mod terrain;

use bevy_log::LogPlugin;
use bevy_tui::{prelude::{initialize_terminal, teardown_terminal}, MinimalTuiPlugins};

use crate::prelude::*;

fn main() {
    let _ = initialize_terminal();

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(MinimalTuiPlugins);
    app.add_plugins(LogPlugin::default());
    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(networking::NetworkingPlugin);
    app.add_plugins(terrain::TerrainPlugin);
    app.add_plugins(chat::ChatPlugin);
    app.add_plugins(dashboard::DashboardPlugin);
    app.run();

    let _ = teardown_terminal();
}
