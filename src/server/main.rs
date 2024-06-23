pub mod networking;
pub mod player;
pub mod prelude;
pub mod terrain;

use crate::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(networking::NetworkingPlugin);
    app.add_plugins(terrain::TerrainPlugin);
    app.run();
}
