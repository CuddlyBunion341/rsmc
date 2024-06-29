use crate::prelude::*;

static COLLIDER_GRID_SIZE: u32 = 3;
static COLLIDER_RESTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

pub fn setup_coliders_system(mut commands: Commands) {
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
                    .insert(collider_components::MyCollider { key });
            }
        }
    }
}

pub fn handle_collider_update_events_system(
    mut collider_grid_events: EventReader<collider_events::ColliderUpdateEvent>,
    mut query: Query<(&mut Transform, &collider_components::MyCollider)>,
    mut chunk_manager: ResMut<terrain_resources::ChunkManager>,
) {
    for event in collider_grid_events.read() {
        let event_position =
            Vec3::new(event.position[0], event.position[1], event.position[2]).floor();
        for (mut transform, collider) in query.iter_mut() {
            let relative_position = relative_colider_position(collider.key);
            let collider_position = (event_position + relative_position).floor();
            let block = chunk_manager.get_block(collider_position);

            match block {
                Some(block) => {
                    if block != BlockId::Air {
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
