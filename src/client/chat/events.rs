use crate::prelude::*;

#[derive(Event)]
pub struct ChatSyncEvent(pub Vec<lib::ChatMessage>);

#[derive(Event)]
pub struct SendMessageEvent(pub String);

#[derive(Event)]
pub struct ChatFocusEvent();

#[derive(Event)]
pub struct ChatUnfocusEvent();
