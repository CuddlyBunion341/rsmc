use crate::prelude::*;

pub fn receive_message_system(
    mut server: ResMut<RenetServer>,
    mut player_states: ResMut<player_resources::PlayerStates>,
) {
    for client_id in server.clients_id() {
        let message_bytes = server.receive_message(client_id, DefaultChannel::ReliableUnordered);

        if message_bytes.is_none() {
            warn!("Failed to receive message.");
            continue;
        }

        let some_message = bincode::deserialize(&message_bytes.unwrap());

        if some_message.is_err() {
            warn!("Failed to deserialize message.");
            continue;
        }

        let message = some_message.unwrap();

        match message {
            lib::NetworkingMessage::PlayerUpdate(player) => {
                info!(
                    "Received player update from client {} {}",
                    client_id, player.position
                );
                player_states.players.insert(client_id, player);
            }
            _ => {
                warn!("Received unknown message type.");
            }
        }
    }
}

pub fn handle_events_system(
    mut server: ResMut<RenetServer>,
    mut server_events: EventReader<ServerEvent>,
    mut player_states: ResMut<player_resources::PlayerStates>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
                player_states.players.insert(
                    *client_id,
                    lib::PlayerState {
                        position: Vec3::ZERO,
                        rotation: Quat::IDENTITY,
                    },
                );
                let message =
                    bincode::serialize(&lib::NetworkingMessage::PlayerJoin(*client_id)).unwrap();
                server.broadcast_message_except(
                    *client_id,
                    DefaultChannel::ReliableOrdered,
                    message,
                );
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
                player_states.players.remove(client_id);
                let message =
                    bincode::serialize(&lib::NetworkingMessage::PlayerLeave(*client_id)).unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
        }
    }
}
