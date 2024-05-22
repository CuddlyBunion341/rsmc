use bevy::{
    pbr::{light_consts, CascadeShadowConfigBuilder},
    prelude::*,
    window::WindowResolution,
};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use chunk_manager::ChunkManager;
use input::handle_mouse_events;
use physics::setup_physics;
use raycaster::{add_highlight_cube, raycast, BlockSelection, SelectedNormal, SelectedPosition};
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

use iyes_perf_ui::prelude::*;
use std::f32::consts::PI;
use world::setup_world;

mod blocks;
mod chunk;
mod chunk_manager;
mod generator;
mod input;
mod mesher;
mod physics;
mod raycaster;
mod world;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1920.0, 1080.0)
                            .with_scale_factor_override(2.0),
                        present_mode: bevy::window::PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LookTransformPlugin)
        .add_plugins(FpsCameraPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PerfUiPlugin)
        .insert_resource(ChunkManager::new())
        .insert_resource(SelectedPosition(None))
        .insert_resource(SelectedNormal(None))
        .insert_resource(BlockSelection {
            position: None,
            normal: None,
        })
        .add_systems(
            Startup,
            (setup, setup_world, add_highlight_cube, setup_physics),
        )
        .add_systems(Update, (raycast, handle_mouse_events))
        .run();
}

#[derive(Component)]
struct MyCube;

#[derive(Component)]
pub struct MyChunk {
    pub position: [i32; 3],
}

fn setup(mut commands: Commands) {
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
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            maximum_distance: 256.0,
            ..default()
        }
        .into(),
        ..default()
    });

    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                translate_sensitivity: 10.0,
                ..default()
            },
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}
