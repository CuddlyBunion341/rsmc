use std::f32::consts::TAU;

use bevy::{prelude::*, render::camera::Exposure, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

use bevy_fps_controller::controller::*;

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.0, 0.0);

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 10000.0,
        })
        .insert_resource(ClearColor(Color::hex("D4F5F5").unwrap()))
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
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FpsControllerPlugin)
        .add_systems(Startup, (setup, setup_scene))
        .add_systems(Update, manage_cursor)
        .run();
}

fn setup(mut commands: Commands, mut window: Query<&mut Window>) {
    let mut window = window.single_mut();
    window.title = String::from("Minimal FPS Controller Example");

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 7.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

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
            exposure: Exposure::SUNLIGHT,
            ..default()
        },
        RenderPlayer { logical_entity },
    ));
}

fn setup_scene(mut commands: Commands) {
    // add a ground plane
    commands.spawn((
        Collider::cuboid(10.0, 0.1, 10.0),
        RigidBody::Fixed,
        Transform::from_translation(Vec3::ZERO),
    ));
}

fn manage_cursor(
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
