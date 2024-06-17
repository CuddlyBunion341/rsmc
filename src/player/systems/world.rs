use bevy::ecs::{event::{EventReader, EventWriter}, system::ResMut};

use crate::terrain::{events::{BlockUpdateEvent, ChunkMeshUpdateEvent}, resources::ChunkManager, util::chunk::CHUNK_SIZE};

pub fn handle_block_update_events(
    mut chunk_manager: ResMut<ChunkManager>,
    mut block_update_events: EventReader<BlockUpdateEvent>,
    mut chunk_mesh_update_events: EventWriter<ChunkMeshUpdateEvent>,
) {
    for event in block_update_events.read() {
        chunk_manager.set_block(event.position, event.block);
        chunk_mesh_update_events.send(ChunkMeshUpdateEvent {
            position: event.position / CHUNK_SIZE as f32,
        });
    }
}

