use std::f32::consts::TAU;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

use crate::{input::LastPlayerPosition, physics::ColliderUpdateEvent};

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 60.0, 0.0);

pub fn setup_controller(mut commands: Commands, mut window: Query<&mut Window>) {
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

pub fn manage_cursor(
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

pub fn handle_controller_movement(
    query: Query<(Entity, &FpsControllerInput, &Transform)>,
    mut last_position: ResMut<LastPlayerPosition>,
    mut collider_events: EventWriter<ColliderUpdateEvent>,
) {
    for (entity, input, transform) in &mut query.iter() {
        let controller_position = transform.translation;
        if last_position.0.floor() != controller_position.floor() {
            collider_events.send(ColliderUpdateEvent {
                position: controller_position.into(),
            });
        }
        last_position.0 = controller_position;
    }
}
