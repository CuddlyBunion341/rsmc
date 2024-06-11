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

use crate::{
    blocks::BlockId,
    chunk::{self, Chunk, CHUNK_SIZE}, my_bevy::{components::{ChunkMesh, HighlightCube}, events::{BlockUpdateEvent, ChunkMeshUpdateEvent, ColliderUpdateEvent}, resources::{BlockSelection, ChunkManager}}, world::add_chunk_objects,
};

pub fn handle_mouse_events(
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

pub fn handle_keyboard_events(
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

fn set_block(position: Vec3, block: BlockId, chunk_manager: &mut ChunkManager) {
    match chunk_from_selection(position, chunk_manager) {
        Some(chunk) => {
            let chunk_position = Vec3::new(
                chunk.position[0] as f32 * chunk::CHUNK_SIZE as f32,
                chunk.position[1] as f32 * chunk::CHUNK_SIZE as f32,
                chunk.position[2] as f32 * chunk::CHUNK_SIZE as f32,
            );
            let local_position = (position - chunk_position).floor();
            chunk.set(
                local_position.x as usize,
                local_position.y as usize,
                local_position.z as usize,
                block,
            );
        }
        None => {
            println!("No chunk found");
        }
    }
}

pub fn get_block(position: Vec3, chunk_manager: &mut ChunkManager) -> Option<BlockId> {
    match chunk_from_selection(position, chunk_manager) {
        Some(chunk) => {
            let chunk_position = Vec3::new(
                chunk.position[0] as f32 * chunk::CHUNK_SIZE as f32,
                chunk.position[1] as f32 * chunk::CHUNK_SIZE as f32,
                chunk.position[2] as f32 * chunk::CHUNK_SIZE as f32,
            );
            let local_position = (position - chunk_position).floor();
            Some(chunk.get(
                local_position.x as usize,
                local_position.y as usize,
                local_position.z as usize,
            ))
        }
        None => {
            println!("No chunk found for block at {:?}", position);
            None
        }
    }
}
