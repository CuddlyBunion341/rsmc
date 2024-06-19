use crate::prelude::*;

pub fn handle_block_update_events(
    mut chunk_manager: ResMut<terrain_resources::ChunkManager>,
    mut block_update_events: EventReader<terrain_events::BlockUpdateEvent>,
    mut chunk_mesh_update_events: EventWriter<terrain_events::ChunkMeshUpdateEvent>,
) {
    for event in block_update_events.read() {
        chunk_manager.set_block(event.position, event.block);
        chunk_mesh_update_events.send(terrain_events::ChunkMeshUpdateEvent {
            position: event.position / CHUNK_SIZE as f32,
        });
    }
}
