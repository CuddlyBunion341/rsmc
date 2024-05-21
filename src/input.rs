use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        event::EventReader,
        query::{With, Without},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    gizmos::gizmos::Gizmos,
    input::mouse::{MouseButton, MouseButtonInput},
    math::{primitives::Cuboid, Ray3d, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    prelude::{default, Deref, DerefMut},
    render::{color::Color, mesh::Mesh},
    transform::components::Transform,
};
use bevy_mod_raycast::immediate::{Raycast, RaycastSettings};
use smooth_bevy_cameras::controllers::fps::FpsCameraController;

use crate::{
    chunk::{self, CHUNK_SIZE},
    chunk_manager::ChunkManager,
    raycaster::{HighlightCube, SelectedPosition},
};

pub fn handle_mouse_events(
    mut events: EventReader<MouseButtonInput>,
    selected_position: Res<SelectedPosition>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    if (selected_position.0).is_none() {
        return;
    }

    for event in events.read() {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            break_block(selected_position.unwrap(), chunk_manager.as_mut())
        }
    }
}

fn break_block(position: Vec3, chunk_manager: &mut ChunkManager) {
    set_block(position, 0, chunk_manager)
}

fn set_block(position: Vec3, block: u8, chunk_manager: &mut ChunkManager) {
    let chunk_position = position / CHUNK_SIZE as f32;
    let chunk = chunk_manager.get_chunk(chunk_position);
    match chunk {
        Some(chunk) => {
            let chunk_position = Vec3::new(
                chunk.position[0] as f32 * chunk::CHUNK_SIZE as f32,
                chunk.position[1] as f32 * chunk::CHUNK_SIZE as f32,
                chunk.position[2] as f32 * chunk::CHUNK_SIZE as f32,
            );
            let local_position = (position - chunk_position).floor();
            println!(
                "localpos: {} {} {}",
                local_position.x, local_position.y, local_position.z
            );
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
