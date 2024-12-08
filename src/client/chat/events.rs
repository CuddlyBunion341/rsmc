use crate::prelude::*;

#[derive(Event)]
pub struct ChatSyncEvent(pub Vec<lib::ChatMessage>);

#[derive(Event)]
pub struct SendMessageEvent(pub String);

#[derive(Event)]
pub struct ChatFocusEvent();

pub enum FocusState {
    Focus,
    Unfocus,
}

#[derive(Event)]
pub struct FocusChangeEvent{
    pub state: FocusState
}

#[derive(Event)]
pub struct ChatUnfocusEvent();
