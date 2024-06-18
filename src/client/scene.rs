use std::f32::consts::PI;

use bevy::{ecs::system::Commands, math::{EulerRot, Quat}, pbr::{light_consts, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle}, prelude::default, transform::components::Transform};
use iyes_perf_ui::PerfUiCompleteBundle;

pub fn setup_scene(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.3,
            PI / 2. + 0.3,
            -PI / 4.,
        )),
        cascade_shadow_config: bevy::pbr::CascadeShadowConfig::from(CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            maximum_distance: 256.0,
            ..default()
        }),
        ..default()
    });
}
