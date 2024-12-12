use crate::prelude::*;

pub fn receive_message_system(
    mut server: ResMut<RenetServer>,
    mut player_states: ResMut<player_resources::PlayerStates>,
    mut past_block_updates: ResMut<terrain_resources::PastBlockUpdates>,
    mut chunk_manager: ResMut<terrain_resources::ChunkManager>,
    mut chat_message_events: EventWriter<chat_events::PlayerChatMessageSendEvent>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableUnordered) {
            let message = bincode::deserialize(&message).unwrap();
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
        }

        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
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
                    chat_message_events
                        .send(chat_events::PlayerChatMessageSendEvent { client_id, message });
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
    mut chat_message_events: EventWriter<chat_events::PlayerChatMessageSendEvent>,
    mut chat_sync_events: EventWriter<chat_events::SyncPlayerChatMessagesEvent>,
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

                chat_sync_events.send(chat_events::SyncPlayerChatMessagesEvent {
                    client_id: *client_id,
                });

                chat_message_events.send(chat_events::PlayerChatMessageSendEvent {
                    client_id: lib::SERVER_MESSAGE_ID,
                    message: format!("Player {} joined the game", *client_id),
                });

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

                chat_message_events.send(chat_events::PlayerChatMessageSendEvent {
                    client_id: lib::SERVER_MESSAGE_ID,
                    message: format!("Player {} left the game", client_id),
                });

                let message =
                    bincode::serialize(&lib::NetworkingMessage::PlayerLeave(*client_id)).unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
        }
    }
}
