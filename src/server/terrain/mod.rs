pub mod events;
pub mod resources;
pub mod systems;

use crate::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::BlockUpdateEvent>();
        app.insert_resource(resources::BlockUpdateResource::new());
    }
}
