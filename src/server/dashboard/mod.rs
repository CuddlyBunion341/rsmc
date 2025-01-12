use crate::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut App) {
        info!("Building DashboardPlugin");

        // Register events
        app.add_event::<events::ExampleEvent>();

        // Add systems
        app.add_systems(Update, systems::example_system);

        // Add resources
        app.insert_resource(resources::ExampleResource::default());
      }
    }
