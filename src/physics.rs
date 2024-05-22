use bevy::{
    ecs::{
        event::{Event, EventReader},
        system::{Commands, Query},
    },
    math::primitives::Cuboid,
    pbr::{PbrBundle, StandardMaterial},
    prelude::default,
    transform::{components::Transform, TransformBundle},
};
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{Collider, Restitution},
};

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

#[derive(Event)]
pub struct ColliderUpdateEvent {
    pub position: [f32; 3],
}

pub fn handle_collider_update(
    mut collider_events: EventReader<ColliderUpdateEvent>,
    mut query: Query<(&mut Transform, &mut Collider)>,
) {
    for event in collider_events.read() {
        for (mut transform, collider) in query.iter_mut() {
            println!("Updating collider at {:?}", event.position);
        }
    }
}
