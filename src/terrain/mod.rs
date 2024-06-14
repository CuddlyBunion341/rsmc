use bevy::app::{App, Plugin, Startup};

mod util;
mod components;
mod resources;
mod systems;

use util::*;
use components::*;
use resources::*;
use systems::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_system);
        app.insert_resource(ChunkManager::new());
        app.add_systems(Setup, (
setup_world_system,
add_chunk_objects_system
                ))
    }
}

fn setup_system() {

}
