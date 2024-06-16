use bevy::app::{App, Plugin, Startup};

mod systems;
pub mod resources;
pub mod components;
pub mod util;
pub mod events;

use components::*;
use resources::*;
use systems::*;
use util::*;
use events::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::new());
        app.add_event::<BlockUpdateEvent>();
        app.add_event::<ChunkMeshUpdateEvent>();
        app.add_systems(Startup, setup_world_system);
    }
}
