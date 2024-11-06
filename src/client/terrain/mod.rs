use crate::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub mod util;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        info!("Building TerrainPlugin");
        app.insert_resource(terrain_resources::ChunkManager::new());
        app.insert_resource(util::TextureManager::new());
        app.add_event::<terrain_events::BlockUpdateEvent>();
        app.add_event::<terrain_events::ChunkMeshUpdateEvent>();
        app.add_systems(Startup, terrain_systems::setup_world_system);
        app.add_systems(Update, terrain_systems::handle_chunk_mesh_update_events);
    }
}
