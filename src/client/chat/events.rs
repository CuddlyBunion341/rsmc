use crate::prelude::*;

#[derive(Event)]
pub struct ChatSyncEvent(pub Vec<ChatMessage>);

#[derive(Event)]
pub struct SingleChatSendEvent(pub ChatMessage);

#[derive(Event)]
pub struct ChatMessageSendEvent(pub String);

#[derive(Event)]
pub struct ChatClearEvent;
