use terrain_util::client_block::block_properties;

use crate::prelude::*;

const COLLIDER_GRID_SIZE: i32 = 4;
const COLLIDER_HALF_SIZE: i32 = COLLIDER_GRID_SIZE / 2;

pub fn setup_coliders_system(mut commands: Commands) {
    let empty_shapes: Vec<(Vec3, Quat, Collider)> = vec![];
    commands.spawn((
        collider_components::TerrainCollider,
        RigidBody::Static,
        Collider::compound(empty_shapes),
        CollisionLayers::new(GameLayer::Terrain, [GameLayer::Player]),
        Transform::default(),
    ));
}

pub fn handle_collider_update_events_system(
    mut collider_grid_events: MessageReader<collider_events::ColliderUpdateEvent>,
    mut query: Query<&mut Collider, With<collider_components::TerrainCollider>>,
    chunk_manager: Res<ChunkManager>,
) {
    for event in collider_grid_events.read() {
        let center = IVec3::new(
            event.grid_center_position[0] as i32,
            event.grid_center_position[1] as i32,
            event.grid_center_position[2] as i32,
        );

        let mut shapes: Vec<(Vec3, Quat, Collider)> = Vec::with_capacity(
            (COLLIDER_GRID_SIZE * COLLIDER_GRID_SIZE * COLLIDER_GRID_SIZE) as usize,
        );

        for x in -COLLIDER_HALF_SIZE..COLLIDER_HALF_SIZE {
            for y in -COLLIDER_HALF_SIZE..COLLIDER_HALF_SIZE {
                for z in -COLLIDER_HALF_SIZE..COLLIDER_HALF_SIZE {
                    let block_pos = center + IVec3::new(x, y, z);

                    if let Some(block) = chunk_manager.get_block(block_pos) {
                        if block_properties(block).has_collider {
                            let position = Vec3::new(
                                block_pos.x as f32 + 0.5,
                                block_pos.y as f32 + 0.5,
                                block_pos.z as f32 + 0.5,
                            );
                            shapes.push((position, Quat::IDENTITY, Collider::cuboid(0.5, 0.5, 0.5)));
                        }
                    }
                }
            }
        }

        if let Ok(mut collider) = query.single_mut() {
            *collider = Collider::compound(shapes);
        }
    }
}

#[cfg(test)]
mod tests {
    use collider_events::ColliderUpdateEvent;

    use super::*;

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app
    }

    #[test]
    fn test_setup_coliders_system() {
        let mut app = setup_app();
        app.add_systems(Startup, setup_coliders_system);

        app.update();

        let mut colliders_query = app
            .world_mut()
            .query::<&collider_components::TerrainCollider>();
        let colliders_count = colliders_query.iter(app.world_mut()).count();

        assert_eq!(colliders_count, 1);
    }

    #[test]
    fn test_handle_collider_update_events_system() {
        let mut app = App::new();

        app.add_message::<collider_events::ColliderUpdateEvent>();
        app.add_systems(Update, handle_collider_update_events_system);
        app.insert_resource(ChunkManager::new());

        let empty_shapes: Vec<(Vec3, Quat, Collider)> = vec![];
        app.world_mut().spawn((
            collider_components::TerrainCollider,
            Collider::compound(empty_shapes),
        ));

        let block = BlockId::Dirt;
        let mut resource = app.world_mut().get_resource_mut::<ChunkManager>().unwrap();
        let chunks = ChunkManager::instantiate_chunks(IVec3::ZERO, IVec3::ONE);
        resource.insert_chunks(chunks);
        resource.update_block(IVec3 { x: 6, y: 7, z: 8 }, block);

        app.world_mut().write_message(ColliderUpdateEvent {
            grid_center_position: [6.0, 7.0, 8.0],
        });

        app.update();

        let mut collider_query = app
            .world_mut()
            .query::<&Collider>();
        let world_mut = app.world_mut();
        let collider = collider_query.single(world_mut).unwrap();

        assert!(collider.shape().as_compound().is_some());
    }
}
