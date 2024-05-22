use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        entity::Entity,
        event::EventReader,
        system::{Commands, Query, Res, ResMut},
    },
    input::mouse::{MouseButton, MouseButtonInput},
    math::Vec3,
    pbr::StandardMaterial,
    render::mesh::Mesh,
};

use crate::{
    chunk::{self, Chunk, CHUNK_SIZE},
    chunk_manager::ChunkManager,
    mesher::ChunkMesh,
    raycaster::{BlockSelection},
    world::add_chunk_objects,
};

pub fn handle_mouse_events(
    mut events: EventReader<MouseButtonInput>,
    block_selection: Res<BlockSelection>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mesh_query: Query<(Entity, &ChunkMesh)>,
) {
    if block_selection.normal.is_none() || block_selection.position.is_none() {
        return;
    }

    let position = block_selection.position.unwrap();
    let normal = block_selection.normal.unwrap();

    for event in events.read() {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            break_block(position, chunk_manager.as_mut());

            match chunk_from_selection(position, chunk_manager.as_mut()) {
                Some(chunk) => {
                    for (entity, chunk_mesh) in mesh_query.iter_mut() {
                        if chunk_mesh.key[0] == chunk.position.x as i32
                            && chunk_mesh.key[1] == chunk.position.y as i32
                            && chunk_mesh.key[2] == chunk.position.z as i32
                        {
                            commands.entity(entity).despawn();
                        }
                    }
                    add_chunk_objects(
                        &mut commands,
                        &asset_server,
                        &mut meshes,
                        &mut materials,
                        chunk,
                    );
                }
                None => {
                    println!("No chunk found");
                }
            }
        } else if event.button == MouseButton::Right && event.state.is_pressed() {
            set_block(position + normal, 3, chunk_manager.as_mut());
            let chunk = chunk_from_selection(position, chunk_manager.as_mut());

            match chunk_from_selection(position, chunk_manager.as_mut()) {
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
                        chunk,
                    );
                }
                None => {
                    println!("No chunk found");
                }
            }
        }
    }
}

fn break_block(position: Vec3, chunk_manager: &mut ChunkManager) {
    set_block(position, 0, chunk_manager)
}

fn chunk_from_selection(
    position: Vec3,
    chunk_manager: &mut ChunkManager,
) -> Option<&mut chunk::Chunk> {
    let chunk_position = position / CHUNK_SIZE as f32;
    chunk_manager.get_chunk(chunk_position)
}

fn set_block(position: Vec3, block: u8, chunk_manager: &mut ChunkManager) {
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
