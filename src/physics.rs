use bevy::{
    ecs::{
        component::Component,
        event::{Event, EventReader},
        system::{Commands, Query},
    },
    math::{primitives::Cuboid, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    prelude::default,
    transform::{components::Transform, TransformBundle},
};
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{Collider, Restitution},
};

static COLLIDER_GRID_SIZE: u32 = 5;

#[derive(Component)]
pub struct MyCollider {
    pub key: u32,
}

pub fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 8.0, 0.0)));
}

pub fn add_coliders(mut commands: Commands) {
    let collider_range = 0..COLLIDER_GRID_SIZE;

    for x in collider_range.clone() {
        for y in collider_range.clone() {
            for z in collider_range.clone() {
                let key = x * COLLIDER_GRID_SIZE * COLLIDER_GRID_SIZE + y * COLLIDER_GRID_SIZE + z;
                commands
                    .spawn(Collider::cuboid(1.0, 1.0, 1.0))
                    .insert(TransformBundle::from(Transform::from_xyz(
                        x as f32, y as f32, z as f32,
                    )))
                    .insert(MyCollider { key });
            }
        }
    }
}

#[derive(Event)]
pub struct ColliderUpdateEvent {
    pub position: [f32; 3],
}

pub fn handle_collider_update(
    mut collider_events: EventReader<ColliderUpdateEvent>,
    mut query: Query<(&mut Transform, &mut MyCollider)>,
) {
    for event in collider_events.read() {
        for (mut transform, collider) in query.iter_mut() {
            print!("{:?}", collider.key);
            println!("{:?}", event.position);
            let relative_position = relative_colider_position(collider.key);

            transform.translation = Vec3 {
                x: event.position[0] + relative_position.x,
                y: event.position[1] + relative_position.y,
                z: event.position[2] + relative_position.z,
            }
            .floor();
        }
    }
}

fn relative_colider_position(key: u32) -> Vec3 {
    let x = key / (COLLIDER_GRID_SIZE * COLLIDER_GRID_SIZE);
    let y = (key % (COLLIDER_GRID_SIZE * COLLIDER_GRID_SIZE)) / COLLIDER_GRID_SIZE;
    let z = key % COLLIDER_GRID_SIZE;

    Vec3 {
        x: x as f32 - COLLIDER_GRID_SIZE as f32 / 2.0,
        y: y as f32 - COLLIDER_GRID_SIZE as f32 / 2.0,
        z: z as f32 - COLLIDER_GRID_SIZE as f32 / 2.0,
    }
}
