use crate::prelude::*;

pub mod events;
pub mod resources;
pub mod systems;
pub mod util;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(terrain_resources::ChunkManager::new());
        app.insert_resource(util::TextureManager::new());
        app.add_event::<terrain_events::BlockUpdateEvent>();
        app.insert_resource(resources::PastBlockUpdates::new());
        app.add_systems(Startup, terrain_systems::setup_world_system);
    }
}
