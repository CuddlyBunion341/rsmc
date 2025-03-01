use crate::prelude::*;

pub mod events;
pub mod resources;
pub mod systems;
pub mod util;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::new());
        app.add_event::<terrain_events::BlockUpdateEvent>();
        app.insert_resource(resources::PastBlockUpdates::new());
        app.add_systems(Startup, terrain_systems::setup_world_system);
        app.insert_resource(resources::Generator::default());

        #[cfg(feature = "generator_visualizer")]
        {
            app.insert_resource(resources::NoiseTextureList::default());
            app.add_systems(Startup, terrain_systems::prepare_visualizer_texture_system);
            app.add_systems(Update, terrain_systems::render_visualizer_system);
            app.add_systems(Update, terrain_systems::regenerate_heightmap_system);
            app.add_systems(Update, terrain_systems::handle_regenerate_event_system);

            app.add_event::<terrain_events::RegenerateHeightMapEvent>();
            app.add_event::<terrain_events::WorldRegenerateEvent>();
        }
    }
}
