use bevy::{
    pbr::{light_consts, CascadeShadowConfigBuilder, NotShadowCaster, NotShadowReceiver},
    prelude::*,
    reflect::TypePath,
    render::{
        camera,
        render_resource::{AsBindGroup, ShaderRef},
    },
    window::WindowResolution,
};
use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, Smoother};

use cgmath::{Matrix4, Rad, Vector3};
use iyes_perf_ui::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(720.0, 720.0),
                present_mode: bevy::window::PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LookTransformPlugin)
        .add_plugins(FpsCameraPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct MyCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PerfUiCompleteBundle::default());

    let perlin = Perlin::new(1);
    let chunk_size = 32;

    for x in 0..chunk_size {
        for z in 0..chunk_size {
            let mesh = meshes.add(Cuboid::default());
            let height = perlin.get([x as f64 / 10.0, z as f64 / 10.0]) * 3.0;
            let transform = Transform::from_xyz(
                (x - chunk_size / 2) as f32,
                height.floor() as f32,
                (z - chunk_size / 2) as f32,
            );
            let material = materials.add(StandardMaterial { ..default() });

            commands.spawn((
                MaterialMeshBundle {
                    mesh,
                    transform,
                    material,
                    ..default()
                },
                MyCube,
            ));
        }
    }

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(5.0, 5.0, 0.0),
        point_light: PointLight {
            intensity: 0.0,
            range: 500.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.1,
            PI / 2.,
            -PI / 4.,
        )),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            maximum_distance: 25.0,
            ..default()
        }
        .into(),
        ..default()
    });

    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}
