pub mod components;
pub mod events;
pub mod systems;

use bevy::app::{App, Plugin, Startup, Update};
use systems::*;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_coliders_system);
        app.add_systems(Update, handle_collider_update_events_system);
    }
}
