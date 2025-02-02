use crate::prelude::*;

// const SPAWN_POINT: Vec3 = Vec3::new(0.0, 32.0, 0.0);
const SPAWN_POINT: Vec3 = Vec3::new(128.0, 64.0, -128.0);

pub fn setup_player_camera(mut commands: Commands) {
    commands.spawn((
            Name::new("Player cam?"),
        Camera3d::default(),
        // Projection::Perspective(PerspectiveProjection {
        //     fov: TAU / 5.0,
        //     ..default()
        // }),
        Projection::Orthographic(
            OrthographicProjection {
                scale: 0.125,
                near: 0.0001,
                far: 1000.0,
                viewport_origin: Vec2::new(0.5, 0.5),
                scaling_mode: ScalingMode::WindowSize,
                area: Rect::new(-1.0, -1.0, 1.0, 1.0),
            }
            .into(),
        ),
        RenderPlayer {
            logical_entity: Entity::from_raw(0),
        },
        player_components::PlayerCamera,
    ));
}

pub fn setup_controller_on_area_ready_system(
    mut commands: Commands,
    mut player_spawned: ResMut<player_resources::PlayerSpawned>,
    mut render_player: Query<&mut RenderPlayer>,
) {
    info!("Setting up controller");

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
            RigidBody::Fixed,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            Transform::from_translation(SPAWN_POINT),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 20.0,
                yaw: TAU * 5.0 / 12.0,
                ..default()
            },
            FpsController {
                upright_height: 1.25,
                height: 1.0,
                crouch_height: 0.6,
                air_acceleration: 80.0,
                radius: 0.75,
                ..default()
            },
        ))
        .insert(CameraConfig { height_offset: 0.0 })
        .insert(player_components::Player)
        .id();

    let mut player = render_player.single_mut();
    player.logical_entity = logical_entity;

    player_spawned.0 = true;
}

pub fn handle_controller_movement_system(
    query: Query<(Entity, &FpsControllerInput, &Transform)>,
    mut last_position: ResMut<player_resources::LastPlayerPosition>,
    mut collider_events: EventWriter<collider_events::ColliderUpdateEvent>,
) {
    for (_entity, _input, transform) in &mut query.iter() {
        let controller_position = transform.translation;
        if last_position.0.floor() != controller_position.floor() {
            collider_events.send(collider_events::ColliderUpdateEvent {
                grid_center_position: controller_position.floor().into(),
            });
        }
        last_position.0 = controller_position.floor();
    }
}

pub fn activate_fps_controller_system(mut controller_query: Query<&mut FpsController>) {
    for mut controller in &mut controller_query.iter_mut() {
        controller.enable_input = true;
    }
}

pub fn lock_cursor_system(mut window_query: Query<&mut Window>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}

pub fn deactivate_fps_controller_system(mut controller_query: Query<&mut FpsController>) {
    for mut controller in &mut controller_query.iter_mut() {
        controller.enable_input = false;
    }
}
