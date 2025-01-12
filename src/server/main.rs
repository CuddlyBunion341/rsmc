pub mod chat;
pub mod networking;
pub mod player;
pub mod prelude;
pub mod terrain;
pub mod dashboard;

use bevy_log::LogPlugin;

use crate::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(LogPlugin::default());
    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(networking::NetworkingPlugin);
    app.add_plugins(terrain::TerrainPlugin);
    app.add_plugins(chat::ChatPlugin);
    app.add_plugins(dashboard::DashboardPlugin);
    app.run();
}
