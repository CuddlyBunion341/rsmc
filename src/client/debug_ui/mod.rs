pub mod resources;
pub mod systems;
pub mod components;

use crate::prelude::*;

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::update_debug_ui_system);

        // add text ui elemnet

        app.insert_resource(resources::DebugUi::new());
    }
}
