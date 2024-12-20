use crate::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct SandPlugin;

impl Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        info!("Building SandPlugin");

        app.add_systems(Startup, systems::spawn_falling_blocks_system);
        app.add_systems(
            Update,
            (
                systems::spawn_falling_blocks_system,
                systems::tick_falling_blocks_system,
                systems::remove_old_entities_system,
            ),
        );
    }
}
