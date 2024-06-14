use bevy::{
    app::{App, Plugin, Startup},
    math::Vec3,
};
use bevy_fps_controller::controller::FpsControllerPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode},
    render::RapierDebugRenderPlugin,
};

mod components;
mod resources;
mod systems;

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
                handle_block_update_events,
                handle_chunk_mesh_update_events,
                chunk_from_selection,
                raycast_system,
            ),
        );
    }
}
