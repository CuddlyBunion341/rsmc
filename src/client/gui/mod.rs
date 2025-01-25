pub mod components;
pub mod events;
pub mod systems;

#[cfg(feature = "debug_ui")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::prelude::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, gui_systems::setup_gui_system);

        #[cfg(feature = "debug_ui")]
        {
            app.add_plugins(WorldInspectorPlugin::default());
            app.add_systems(Update, gui_systems::handle_debug_state_transition_system);

            app.add_systems(
                OnEnter(GameState::Debugging),
                gui_systems::handle_enter_debug_state_system,
            );
        }
    }
}
