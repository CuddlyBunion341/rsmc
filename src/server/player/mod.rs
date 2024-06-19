use crate::prelude::*;

pub mod systems;
pub mod resources;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(player_resources::PlayerStates::new());
        app.add_systems(Update, player_systems::broadcast_player_attributes_system);
    }
}
