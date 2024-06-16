use bevy::app::*;
use bevy::math::*;
use bevy_fps_controller::controller::FpsControllerPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode},
    render::RapierDebugRenderPlugin,
};

mod components;
mod events;
mod resources;
mod systems;

use resources::*;
use systems::*;
use events::*;

use crate::collider::events::ColliderUpdateEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsControllerPlugin);
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.insert_resource(RapierConfiguration {
            gravity: Vec3::new(0., -1.6, 0.),
            physics_pipeline_active: true,
            force_update_from_transform_changes: false,
            scaled_shape_subdivision: 1,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Fixed {
                dt: 1.0 / 120.0,
                substeps: 1,
            },
        });
        app.insert_resource(BlockSelection::new());
        app.insert_resource(LastPlayerPosition::new());
        app.add_event::<ColliderUpdateEvent>();
        app.add_systems(
            Startup,
            (setup_controller_system, setup_highlight_cube_system),
        );
        app.add_systems(
            Update,
            (
                handle_controller_movement_system,
                manage_cursor_system,
                handle_mouse_events_system,
                handle_keyboard_events_system,
                raycast_system,
            ),
        );
    }
}