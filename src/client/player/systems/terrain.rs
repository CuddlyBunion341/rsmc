use hello_bevy::NetworkingMessage;

use crate::prelude::*;

pub fn handle_block_update_events(
    mut chunk_manager: ResMut<terrain_resources::ChunkManager>,
    mut block_update_events: EventReader<terrain_events::BlockUpdateEvent>,
    mut chunk_mesh_update_events: EventWriter<terrain_events::ChunkMeshUpdateEvent>,
    mut client: ResMut<RenetClient>,
) {
    for event in block_update_events.read() {
        chunk_manager.set_block(event.position, event.block);
        info!("Block update message: {:?}", event.position);

        chunk_mesh_update_events.send(terrain_events::ChunkMeshUpdateEvent {
            position: event.position / CHUNK_SIZE as f32,
        });

        if !event.from_network {
            client.send_message(
                // TODO: Change channel to ReliableOrdered
                DefaultChannel::ReliableUnordered,
                bincode::serialize(&NetworkingMessage::BlockUpdate {
                    position: event.position,
                    block: event.block,
                })
                .unwrap(),
            );
        }
    }
}
