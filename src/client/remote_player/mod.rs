pub mod systems; 
pub mod components;
pub mod resources;

use crate::prelude::*;

pub struct RemotePlayerPlugin;

impl Plugin for RemotePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                remote_player_systems::spawn_remote_player_system,
                remote_player_systems::update_remote_player_system,
            ),
        );
    }
}
