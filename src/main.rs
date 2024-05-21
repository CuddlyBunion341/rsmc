use bevy::{
    ecs::entity,
    pbr::{light_consts, CascadeShadowConfigBuilder},
    prelude::*,
    utils::hashbrown::Equivalent,
    window::WindowResolution,
};
use bevy_mod_raycast::prelude::*;
use chunk_manager::ChunkManager;
use mesher::{create_cube_geometry_data, create_cube_mesh_from_data};
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

use iyes_perf_ui::prelude::*;
use std::f32::consts::PI;
use world::setup_world;

use crate::chunk::CHUNK_SIZE;

mod blocks;
mod chunk;
mod chunk_manager;
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
        .insert_resource(ChunkManager::new())
        .add_systems(Startup, (setup, setup_world, add_highlight_cube))
        .add_systems(Update, raycast)
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

const RAY_DIST: Vec3 = Vec3::new(0.0, 0.0, -20.0);

// query camera position and direction
fn raycast(
    mut raycast: Raycast,
    mut gizmos: Gizmos,
    query: Query<&Transform, With<FpsCameraController>>,
    mut highlight_query: Query<(&mut Transform, &HighlightCube), Without<FpsCameraController>>,
    mut chunk_manager: ResMut<ChunkManager>,
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
    let selected_position = intersections
        .first()
        .map(|(_, intersection)| {
            (intersection.position() - intersection.normal() * 0.5).floor() + 0.5
        })
        .unwrap_or_else(|| Vec3::ZERO);
    highlight_transform.translation = selected_position;

    println!(
        "Position: {} {} {}",
        selected_position.x, selected_position.y, selected_position.z
    );

    let chunk_position = selected_position / CHUNK_SIZE as f32;
    let chunk = chunk_manager.get_chunk(chunk_position);
    match chunk {
        Some(chunk) => {
            let chunk_position = Vec3::new(
                chunk.position[0] as f32 * chunk::CHUNK_SIZE as f32,
                chunk.position[1] as f32 * chunk::CHUNK_SIZE as f32,
                chunk.position[2] as f32 * chunk::CHUNK_SIZE as f32,
            );
            let local_position = (selected_position - chunk_position).floor();
            println!(
                "localpos: {} {} {}",
                local_position.x, local_position.y, local_position.z
            );
            let block_id = chunk.get(
                local_position.x as usize,
                local_position.y as usize,
                local_position.z as usize,
            );
            chunk.set(
                local_position.x as usize,
                local_position.y as usize,
                local_position.z as usize,
                0,
            );
            println!("Block ID: {}", block_id);
        }
        None => {
            println!("No chunk found");
        }
    }
}

#[derive(Component)]
struct HighlightCube;

fn add_highlight_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Cuboid::new(1.01, 1.01, 1.01);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.5)),
            transform: Transform::from_xyz(0.0, 0.0, -7.0),
            ..default()
        })
        .insert(HighlightCube);
}
