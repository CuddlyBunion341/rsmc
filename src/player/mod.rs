use bevy::{app::{App, Plugin}, math::Vec3};
use bevy_fps_controller::controller::FpsControllerPlugin;
use bevy_rapier3d::{plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode}, render::RapierDebugRenderPlugin};

mod components;
mod systems;
mod resources;

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
    }
}
