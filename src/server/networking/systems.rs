use crate::prelude::*;

pub fn receive_message_system(
    mut server: ResMut<RenetServer>,
    mut player_states: ResMut<player_resources::PlayerStates>,
    mut server_events: EventReader<ServerEvent>,
) {
    for client_id in server.clients_id() {
        let message_bytes = server.receive_message(client_id, DefaultChannel::ReliableUnordered);

        if message_bytes.is_none() {
            continue;
        }

        let some_message = bincode::deserialize(&message_bytes.unwrap());

        if some_message.is_err() {
            println!("Failed to deserialize message.");
            continue;
        }

        let message = some_message.unwrap();
        info!("Received message: {:?}", message);

        match message {
            lib::NetworkingMessage::PlayerUpdate(player) => {
                info!(
                    "Received player update from client {} {}",
                    client_id, player.position
                );
                player_states.players.insert(client_id, player);
            }
            _ => {
                warn!("Received unknown message type. (ReliableUnordered)");
            }
        }

        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let message = bincode::deserialize(&message).unwrap();

            match message {
                lib::NetworkingMessage::BlockUpdate { position, block } => {
                    println!(
                        "Received block update from client {} {} {:?}",
                        client_id, position, block
                    );
                    server.broadcast_message_except(
                        client_id,
                        DefaultChannel::ReliableOrdered,
                        bincode::serialize(&lib::NetworkingMessage::BlockUpdate {
                            position,
                            block,
                        })
                        .unwrap(),
                    );
                }
                _ => {
                    warn!("Received unknown message type. (ReliabelOrdered)");
                }
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
                // TODO: broadcast past block update events to client
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
