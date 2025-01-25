use crate::prelude::*;

#[derive(Event)]
pub struct ChatSyncEvent(pub Vec<lib::ChatMessage>);

#[derive(Event)]
pub struct SingleChatSendEvent(pub lib::ChatMessage);

#[derive(Event)]
pub struct ChatMessageSendEvent(pub String);

#[derive(Event)]
pub struct ChatClearEvent;
