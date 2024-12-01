pub mod components;
pub mod events;
pub mod systems;

use crate::prelude::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, gui_systems::setup_gui_system);
    }
}
