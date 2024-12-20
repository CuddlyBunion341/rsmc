use crate::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct SandPlugin;

impl Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        info!("Building SandPlugin");

        app.add_event::<events::ExampleEvent>();
        app.add_systems(Update, systems::example_system);
        app.insert_resource(resources::ExampleResource::default());
    }
}
