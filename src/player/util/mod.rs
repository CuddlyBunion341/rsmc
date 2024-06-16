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

use crate::terrain::{resources::ChunkManager, util::{blocks::BlockId, chunk}};

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
