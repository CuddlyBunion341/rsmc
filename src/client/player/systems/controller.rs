use crate::prelude::*;

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 20.0, 0.0);

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
        .insert(player_components::Player)
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
    mut last_position: ResMut<player_resources::LastPlayerPosition>,
    mut collider_events: EventWriter<collider_events::ColliderUpdateEvent>,
) {
    for (_entity, _input, transform) in &mut query.iter() {
        let controller_position = transform.translation;
        if last_position.0.floor() != controller_position.floor() {
            collider_events.send(collider_events::ColliderUpdateEvent {
                grid_center_position: controller_position.into(),
            });
        }
        last_position.0 = controller_position;
    }
}
