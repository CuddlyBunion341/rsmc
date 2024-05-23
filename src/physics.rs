use bevy::{
    ecs::{
        component::Component,
        event::{Event, EventReader},
        system::{Commands, Query, ResMut},
    },
    math::Vec3,
    transform::{components::Transform, TransformBundle},
};
use bevy_rapier3d::geometry::Collider;

use crate::{
    chunk_manager::{ChunkManager},
    input::get_block,
};

static COLLIDER_GRID_SIZE: u32 = 5;
static COLLIDER_RESTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Component)]
pub struct MyCollider {
    pub key: u32,
}

pub fn add_coliders(mut commands: Commands) {
    let collider_range = 0..COLLIDER_GRID_SIZE;

    for x in collider_range.clone() {
        for y in collider_range.clone() {
            for z in collider_range.clone() {
                let key = x * COLLIDER_GRID_SIZE * COLLIDER_GRID_SIZE + y * COLLIDER_GRID_SIZE + z;
                commands
                    .spawn(Collider::cuboid(0.5, 0.5, 0.5))
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

pub fn handle_collider_update_events(
    mut collider_grid_events: EventReader<ColliderUpdateEvent>,
    mut query: Query<(&mut Transform, &MyCollider)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for event in collider_grid_events.read() {
        let event_position =
            Vec3::new(event.position[0], event.position[1], event.position[2]).floor();
        for (mut transform, collider) in query.iter_mut() {
            let relative_position = relative_colider_position(collider.key);
            let collider_position = (event_position + relative_position).floor();
            let block = get_block(collider_position, &mut chunk_manager);

            match block {
                Some(block) => {
                    if block != 0 {
                        transform.translation = collider_position + 0.5;
                    } else {
                        transform.translation = COLLIDER_RESTING_POSITION;
                    }
                }
                None => {
                    transform.translation = COLLIDER_RESTING_POSITION;
                }
            }
        }
    }
}

fn relative_colider_position(key: u32) -> Vec3 {
    let x = key / (COLLIDER_GRID_SIZE * COLLIDER_GRID_SIZE);
    let y = (key % (COLLIDER_GRID_SIZE * COLLIDER_GRID_SIZE)) / COLLIDER_GRID_SIZE;
    let z = key % COLLIDER_GRID_SIZE;

    Vec3 {
        x: x as f32 - (COLLIDER_GRID_SIZE / 2) as f32,
        y: y as f32 - (COLLIDER_GRID_SIZE / 2) as f32,
        z: z as f32 - (COLLIDER_GRID_SIZE / 2) as f32,
    }
}
