pub mod chat;
pub mod networking;
pub mod player;
pub mod prelude;
pub mod terrain;

use crate::prelude::*;
use bevy_renet::netcode::NetcodeServerPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    // app.add_plugins(LogPlugin::default()); // TODO: integrate logging
    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin);
    app.add_plugins(networking::NetworkingPlugin);
    app.add_plugins(terrain::TerrainPlugin);
    app.add_plugins(chat::ChatPlugin);
    app.run();
}
