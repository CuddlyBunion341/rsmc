use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButton, MouseButtonInput},
    },
    math::Vec3,
    pbr::StandardMaterial,
    render::mesh::Mesh,
    transform::components::Transform,
};

use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Query, ResMut, Resource},
    },
    gizmos::gizmos::Gizmos,
    math::{primitives::Cuboid, Ray3d, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    prelude::{default, Deref, DerefMut},
    render::{camera::Camera, color::Color, mesh::Mesh},
    transform::components::Transform,
};
use bevy_mod_raycast::immediate::{Raycast, RaycastSettings};

const RAY_DIST: Vec3 = Vec3::new(0.0, 0.0, -20.0);

use crate::{
    blocks::BlockId,
    chunk::{self, Chunk, CHUNK_SIZE}, my_bevy::{components::{ChunkMesh, HighlightCube}, events::{BlockUpdateEvent, ChunkMeshUpdateEvent, ColliderUpdateEvent}, resources::{BlockSelection, ChunkManager}}, world::add_chunk_objects,
};
use std::f32::consts::TAU;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

use crate::my_bevy::{events::ColliderUpdateEvent, resources::LastPlayerPosition};

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 256.0, 0.0);

pub fn setup_controller_system(mut commands: Commands, mut window: Query<&mut Window>) {
    let mut window = window.single_mut();
    window.title = String::from("Minimal FPS Controller Example");

    let logical_entity = commands
        .spawn((
            Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(Transform::from_translation(SPAWN_POINT)),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                upright_height: 1.25,
                height: 1.0,
                crouch_height: 0.8,
                air_acceleration: 80.0,
                ..default()
            },
        ))
        .insert(CameraConfig {
            height_offset: 0.0,
            radius_scale: 0.75,
        })
        .id();

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: TAU / 5.0,
                ..default()
            }),
            ..default()
        },
        RenderPlayer { logical_entity },
    ));
}

pub fn handle_controller_movement_system(
    query: Query<(Entity, &FpsControllerInput, &Transform)>,
    mut last_position: ResMut<LastPlayerPosition>,
    mut collider_events: EventWriter<ColliderUpdateEvent>,
) {
    for (_entity, _input, transform) in &mut query.iter() {
        let controller_position = transform.translation;
        if last_position.0.floor() != controller_position.floor() {
            collider_events.send(ColliderUpdateEvent {
                position: controller_position.into(),
            });
        }
        last_position.0 = controller_position;
    }
}

pub fn manage_cursor_system(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    let mut window = window_query.single_mut();
    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
}

pub fn handle_mouse_events_system(
    mut block_update_events: EventWriter<BlockUpdateEvent>,
    mut mouse_events: EventReader<MouseButtonInput>,
    block_selection: Res<BlockSelection>,
) {
    if block_selection.normal.is_none() || block_selection.position.is_none() {
        return;
    }

    let position = block_selection.position.unwrap();
    let normal = block_selection.normal.unwrap();

    for event in mouse_events.read() {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            block_update_events.send(BlockUpdateEvent {
                position,
                block: BlockId::Air,
            });
        } else if event.button == MouseButton::Right && event.state.is_pressed() {
            block_update_events.send(BlockUpdateEvent {
                position: position + normal,
                block: BlockId::Dirt,
            });
        }
    }
}

pub fn handle_keyboard_events_system(
    mut keyboard_events: EventReader<KeyboardInput>,
    camera_query: Query<&Transform, With<HighlightCube>>,
    mut collider_events: EventWriter<ColliderUpdateEvent>,
) {
    for event in keyboard_events.read() {
        if event.state.is_pressed() {
            match event.key_code {
                bevy::input::keyboard::KeyCode::Escape => std::process::exit(0),
                bevy::input::keyboard::KeyCode::KeyC => {
                    let controller_transform = camera_query.single();
                    println!("Handling event: {:?}", controller_transform.translation);
                    collider_events.send(ColliderUpdateEvent {
                        position: controller_transform.translation.into(),
                    });
                }
                _ => {}
            }
        }
    }
}

pub fn handle_block_update_events(
    mut chunk_manager: ResMut<ChunkManager>,
    mut block_update_events: EventReader<BlockUpdateEvent>,
    mut chunk_mesh_update_events: EventWriter<ChunkMeshUpdateEvent>,
) {
    for event in block_update_events.read() {
        set_block(event.position, event.block, chunk_manager.as_mut());
        chunk_mesh_update_events.send(ChunkMeshUpdateEvent {
            position: event.position / CHUNK_SIZE as f32,
        });
    }
}

pub fn handle_chunk_mesh_update_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_mesh_update_events: EventReader<ChunkMeshUpdateEvent>,
    mut mesh_query: Query<(Entity, &ChunkMesh)>,
) {
    for event in chunk_mesh_update_events.read() {
        let chunk_option = chunk_manager.get_chunk(event.position);
        match chunk_option {
            Some(chunk) => {
                for (entity, chunk_mesh) in mesh_query.iter_mut() {
                    if Chunk::key_eq_pos(chunk_mesh.key, chunk.position) {
                        commands.entity(entity).despawn();
                    }
                }
                add_chunk_objects(
                    &mut commands,
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    &chunk,
                );
            }
            None => {
                println!("No chunk found");
            }
        }
    }
}

fn chunk_from_selection(
    position: Vec3,
    chunk_manager: &mut ChunkManager,
) -> Option<&mut chunk::Chunk> {
    let chunk_position = position / CHUNK_SIZE as f32;
    chunk_manager.get_chunk(chunk_position)
}

// query camera position and direction
pub fn raycast_system(
    mut raycast: Raycast,
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Camera>>,
    mut highlight_query: Query<(&mut Transform, &HighlightCube), Without<Camera>>,
    mut block_selection: ResMut<BlockSelection>,
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
    let hover_position = intersections
        .first()
        .map(|(_, intersection)| (intersection.position() - intersection.normal() * 0.5).floor());

    block_selection.position = hover_position.clone();
    block_selection.normal = intersections
        .first()
        .map(|(_, intersection)| intersection.normal());

    if hover_position.is_none() {
        highlight_transform.translation = Vec3::new(-100.0, -100.0, -100.0);
        return;
    }

    highlight_transform.translation = hover_position.unwrap() + 0.5;
}

pub fn setup_highlight_cube_system(
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
