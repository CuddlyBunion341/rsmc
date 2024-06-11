use ::bevy::math::Vec3;
use bevy::{
    pbr::{light_consts, CascadeShadowConfigBuilder},
    prelude::*,
    window::WindowResolution,
};
use bevy_fps_controller::controller::FpsControllerPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode},
    render::RapierDebugRenderPlugin,
};
use controller::{handle_controller_movement, manage_cursor, setup_controller};
use input::{
    handle_block_update_events, handle_chunk_mesh_update_events, handle_keyboard_events,
    handle_mouse_events
};
use my_bevy::{events::{BlockUpdateEvent, ChunkMeshUpdateEvent, ColliderUpdateEvent}, resources::{BlockSelection, ChunkManager, LastPlayerPosition, SelectedNormal, SelectedPosition}};
use physics::{add_coliders, handle_collider_update_events};
use raycaster::{add_highlight_cube, raycast};

use iyes_perf_ui::prelude::*;
use scene::setup_scene;
use world::setup_world;

mod blocks;
mod chunk;
mod chunk_manager;
mod controller;
mod generator;
mod input;
mod mesher;
mod physics;
mod raycaster;
mod world;
mod scene;

mod my_bevy;

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
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec3::new(0., -1.6, 0.),
            physics_pipeline_active: true,
            force_update_from_transform_changes: false,
            scaled_shape_subdivision: 1,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Fixed {
                dt: 1.0 / 120.0,
                substeps: 1,
            },
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PerfUiPlugin)
        .add_plugins(FpsControllerPlugin)
        .insert_resource(ChunkManager::new())
        .insert_resource(SelectedPosition(None))
        .insert_resource(SelectedNormal(None))
        .insert_resource(BlockSelection {
            position: None,
            normal: None,
        })
        .insert_resource(LastPlayerPosition(Vec3::ZERO))
        .add_systems(
            Startup,
            (
                setup_scene,
                setup_world,
                add_highlight_cube,
                add_coliders,
                setup_controller,
            ),
        )
        .add_systems(
            Update,
            (
                raycast,
                handle_mouse_events,
                handle_block_update_events,
                handle_chunk_mesh_update_events,
                handle_keyboard_events,
                handle_collider_update_events,
                manage_cursor,
                handle_controller_movement,
            ),
        )
        .add_event::<BlockUpdateEvent>()
        .add_event::<ChunkMeshUpdateEvent>()
        .add_event::<ColliderUpdateEvent>()
        .run();
}
