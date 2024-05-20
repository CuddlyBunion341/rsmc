use bevy::{
    ecs::entity,
    pbr::{light_consts, CascadeShadowConfigBuilder},
    prelude::*,
    utils::hashbrown::Equivalent,
    window::WindowResolution,
};
use bevy_mod_raycast::prelude::*;
use mesher::{create_cube_geometry_data, create_cube_mesh_from_data};
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

use iyes_perf_ui::prelude::*;
use std::f32::consts::PI;
use world::setup_world;

mod blocks;
mod chunk;
mod generator;
mod mesher;
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
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, (setup, setup_world, add_highlight_cube))
        .add_systems(Update, raycast)
        .run();
}

#[derive(Component)]
struct MyCube;

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

const RAY_DIST: Vec3 = Vec3::new(0.0, 0.0, -20.0);

// query camera position and direction
fn raycast(
    mut raycast: Raycast,
    mut gizmos: Gizmos,
    query: Query<&Transform, With<FpsCameraController>>,
    mut highlight_query: Query<(&mut Transform, &HighlightCube), Without<FpsCameraController>>,
) {
    let camera_transform = query.single();
    let filter = |entity| !highlight_query.contains(entity);

    let pos = camera_transform.translation;
    let dir = camera_transform.rotation.mul_vec3(Vec3::Z).normalize();
    let dir = dir * RAY_DIST.z;

    let intersections = raycast.debug_cast_ray(
        Ray3d::new(pos, dir),
        &RaycastSettings {
            filter: &filter,
            ..default()
        },
        &mut gizmos,
    );

    let (mut highlight_transform, _) = highlight_query.single_mut();
    highlight_transform.translation = intersections
        .first()
        .map(|(_, intersection)| intersection.position())
        .unwrap_or_else(|| Vec3::ZERO);
}

#[derive(Component)]
struct HighlightCube;

fn add_highlight_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Cuboid::new(1.0, 1.0, 1.0);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 1.0, 0.0),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, -7.0),
            ..default()
        })
        .insert(HighlightCube);
}
