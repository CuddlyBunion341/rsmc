use bevy_rapier3d::na::RealField;

use crate::prelude::*;

pub fn receive_message_system(
    mut server: ResMut<RenetServer>,
    mut player_states: ResMut<player_resources::PlayerStates>,
    mut past_block_updates: ResMut<terrain_resources::PastBlockUpdates>,
    mut chat_messages: ResMut<chat_resources::ChatHistory>,
    mut chunk_manager: ResMut<terrain_resources::ChunkManager>,
) {
    for client_id in server.clients_id() {
        let message_bytes = server.receive_message(client_id, DefaultChannel::ReliableUnordered);

        if message_bytes.is_none() {
            warn!("Failed to receive message. (ReliableUnordered)");
            continue;
        }

        let some_message = bincode::deserialize(&message_bytes.unwrap());

        if some_message.is_err() {
            warn!("Failed to deserialize message.");
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
            lib::NetworkingMessage::ChunkBatchRequest(positions) => {
                info!(
                    "Received chunk batch request at {:?} from client {}",
                    positions, client_id
                );

                let chunks: Vec<Chunk> = positions
                    .into_iter()
                    .map(|position| {
                        let chunk = chunk_manager.get_chunk(position);

                        match chunk {
                            Some(chunk) => *chunk,
                            None => {
                                let mut chunk = lib::Chunk::new(position);

                                let generator = terrain_util::generator::Generator::new(0);

                                generator.generate_chunk(&mut chunk);

                                chunk
                            }
                        }
                    })
                    .collect();

                let message =
                    bincode::serialize(&lib::NetworkingMessage::ChunkBatchResponse(chunks));
                server.send_message(
                    client_id,
                    DefaultChannel::ReliableUnordered,
                    message.unwrap(),
                );
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
                    info!(
                        "Received block update from client {} {} {:?}",
                        client_id, position, block
                    );
                    past_block_updates
                        .updates
                        .push(terrain_events::BlockUpdateEvent { position, block });

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
                lib::NetworkingMessage::ChatMessageSend(message) => {
                    info!("Received chat message from {}", client_id);

                    let message_count = chat_messages.messages.len();
                    let message_id = message_count;

                    chat_messages.messages.push(lib::ChatMessage {
                        client_id,
                        message_id,
                        message,
                        timestamp: 0, // TODO: implement
                    });

                    let response_message = lib::NetworkingMessage::ChatMessageSync(chat_messages.messages.clone());

                    server.broadcast_message(DefaultChannel::ReliableOrdered, bincode::serialize(&response_message).unwrap());
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
    past_block_updates: Res<terrain_resources::PastBlockUpdates>,
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

                for update in past_block_updates.updates.iter() {
                    let message = bincode::serialize(&lib::NetworkingMessage::BlockUpdate {
                        position: update.position,
                        block: update.block,
                    })
                    .unwrap();
                    server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
                }
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
