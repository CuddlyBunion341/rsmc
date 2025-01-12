use crate::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut App) {
        info!("Building DashboardPlugin");

        app.add_event::<events::ExampleEvent>();

        app.add_systems(Update, (systems::run_basic_ui));
        app.add_systems(Update, (systems::quit_system));

        app.insert_resource(resources::ExampleResource::default());
    }
}
