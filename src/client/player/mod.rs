pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building PlayerPlugin");
        info!("Building PlayerPlugin");
        app.add_plugins(FpsControllerPlugin);
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.insert_resource(RapierConfiguration {
            gravity: Vec3::new(0., 0.0, 0.),
            physics_pipeline_active: true,
            force_update_from_transform_changes: false,
            scaled_shape_subdivision: 1,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Fixed {
                dt: 1.0 / 120.0,
                substeps: 1,
            },
        });
        app.add_event::<player_events::PlayerColliderUpdateEvent>();
        app.insert_resource(player_resources::BlockSelection::new());
        app.insert_resource(player_resources::PlayerSpawned(false));
        app.insert_resource(player_resources::LastPlayerPosition::new());
        app.add_systems(Startup, 
            (
                player_systems::setup_highlight_cube_system,
                player_systems::setup_player_camera
            )
        );
        app.add_systems(
            Update,
            (
                player_systems::setup_controller_on_area_ready_system,
            )
            .run_if(terrain_resources::SpawnAreaLoaded::is_loaded)
            .run_if(player_resources::PlayerSpawned::is_not_spawned),
        );
        app.add_systems(
            Update,
            (
                player_systems::handle_controller_movement_system,
                player_systems::manage_cursor_system,
                player_systems::handle_mouse_events_system,
                player_systems::handle_keyboard_events_system,
                player_systems::raycast_system,
                player_systems::handle_block_update_events,
                player_systems::broadcast_player_attributes_system,
                player_systems::handle_player_collider_events_system,
            ).run_if(player_resources::PlayerSpawned::is_spawned),
        );
    }
}
