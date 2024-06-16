use bevy::app::{App, Plugin, Startup, Update};

pub mod components;
pub mod events;
pub mod resources;
mod systems;
pub mod util;

use components::*;
use events::*;
use resources::*;
use systems::*;
use util::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::new());
        app.add_event::<BlockUpdateEvent>();
        app.add_event::<ChunkMeshUpdateEvent>();
        app.add_systems(Startup, setup_world_system);
        app.add_systems(Update, handle_chunk_mesh_update_events);
    }
}
