use crate::prelude::*;

use networking_messages::NetworkingMessage;

pub fn receive_message_system(
    mut client: ResMut<RenetClient>,
    mut player_spawn_events: ResMut<Events<remote_player_events::RemotePlayerSpawnedEvent>>,
    mut player_despawn_events: ResMut<Events<remote_player_events::RemotePlayerDespawnedEvent>>,
    mut player_sync_events: ResMut<Events<remote_player_events::RemotePlayerSyncEvent>>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let message = bincode::deserialize(&message).unwrap();

        match message {
            NetworkingMessage::PlayerJoin(event) => {
                player_spawn_events
                    .send(remote_player_events::RemotePlayerSpawnedEvent { client_id: event });
            }
            NetworkingMessage::PlayerLeave(event) => {
                player_despawn_events
                    .send(remote_player_events::RemotePlayerDespawnedEvent { client_id: event });
            }
            _ => {
                warn!("Received unknown message type.");
            }
        }
    }

    while let Some(message) = client.receive_message(DefaultChannel::ReliableUnordered) {
        let message = bincode::deserialize(&message).unwrap();

        match message {
            NetworkingMessage::PlayerSync(event) => {
                player_sync_events
                    .send(remote_player_events::RemotePlayerSyncEvent { players: event });
            }
            _ => {
                warn!("Received unknown message type.")
            }
        }
    }
}
