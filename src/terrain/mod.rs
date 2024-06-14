use bevy::app::{App, Plugin, Startup};

mod components;
mod resources;
mod systems;
mod util;

use components::*;
use resources::*;
use systems::*;
use util::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_system);
        app.insert_resource(ChunkManager::new());
        app.add_systems(Startup, setup_world_system);
    }
}
