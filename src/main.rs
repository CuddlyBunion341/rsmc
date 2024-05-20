use bevy::{
    pbr::{light_consts, CascadeShadowConfigBuilder},
    prelude::*,
    window::WindowResolution,
};
use mesher::{create_cube_geometry_data, create_cube_mesh_from_data};
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

use iyes_perf_ui::prelude::*;
use std::f32::consts::PI;
use world::setup_world;

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
        .add_systems(Startup, (setup, setup_world, add_iron_ore))
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
            maximum_distance: 25.0,
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

pub fn add_iron_ore(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("textures/copper_block.png");
    let normal_texture_handle = asset_server.load("textures/copper_block_n.png");
    let specular_texture_handle = asset_server.load("textures/copper_block_s.png");

    let mut cube_mesh =
        create_cube_mesh_from_data(create_cube_geometry_data(0.0, 0.0, 0.0, 0b111111));
    let _ = cube_mesh.generate_tangents();

    for x in 0..16 {
        for z in 0..16 {
            commands.spawn((
                MaterialMeshBundle {
                    mesh: meshes.add(cube_mesh.clone()),
                    transform: Transform::from_xyz(x as f32 * 2.0, 30.0, z as f32 * 2.0),
                    material: materials.add(StandardMaterial {
                        metallic: 1.0,
                        perceptual_roughness: 0.0,
                        base_color_texture: Some(texture_handle.clone()),
                        normal_map_texture: Some(normal_texture_handle.clone()),
                        metallic_roughness_texture: Some(specular_texture_handle.clone()),
                        ..default()
                    }),
                    ..default()
                },
                MyCube,
            ));
        }
    }
}
