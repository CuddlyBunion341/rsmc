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
        app.insert_resource(lib::ChunkManager::new());
        app.insert_resource(terrain_resources::SpawnAreaLoaded(false));
        app.insert_resource(util::TextureManager::new());
        app.add_event::<terrain_events::BlockUpdateEvent>();
        app.add_event::<terrain_events::ChunkMeshUpdateEvent>();
        app.add_systems(Startup, terrain_systems::prepare_spawn_area_system);
        app.add_systems(Startup, terrain_systems::generate_world_system);
        app.add_systems(Update, terrain_systems::handle_chunk_mesh_update_events);
    }
}
