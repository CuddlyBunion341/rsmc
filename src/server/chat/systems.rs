use std::time::UNIX_EPOCH;

use crate::prelude::*;

pub fn handle_network_chat_message_send(
    mut server: ResMut<RenetServer>,
    mut player_send_messages: EventReader<chat_events::PlayerChatMessageSendEvent>,
    mut chat_messages: ResMut<chat_resources::ChatHistory>,
) {

    for event in player_send_messages.read() {
        let message = event.message.clone();
        let client_id = event.client_id;
        let message_count = chat_messages.messages.len();
        let message_id = message_count;

        chat_messages.messages.push(lib::ChatMessage {
            client_id,
            message_id,
            message,
            timestamp: get_current_time_in_ms(),
        });

        let response_message =
            lib::NetworkingMessage::ChatMessageSync(chat_messages.messages.clone());

        server.broadcast_message(
            DefaultChannel::ReliableOrdered,
            bincode::serialize(&response_message).unwrap(),
        );
    }
}

fn get_current_time_in_ms() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH);
    match since_the_epoch {
        Ok(time) => match time.as_millis().try_into() {
            Ok(casted_time) => casted_time,
            Err(_error) => {
                error!("Could not cast time milis to u32");
                0
            }
        },
        Err(_error) => {
            error!("Could not fetch system time");
            0
        }
    }
}
