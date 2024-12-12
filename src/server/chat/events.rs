use crate::prelude::*;

#[derive(Event)]
pub struct PlayerChatMessageSendEvent {
    pub client_id: ClientId,
    pub message: String,
}

#[derive(Event)]
pub struct SyncPlayerChatMessagesEvent {
    pub client_id: ClientId,
}
