use crate::prelude::*;

#[derive(Event)]
pub struct ChatSyncEvent(pub Vec<lib::ChatMessage>);

#[derive(Event)]
pub struct SingleChatSendEvent(pub lib::ChatMessage);

#[derive(Event)]
pub struct SendMessageEvent(pub String);

pub enum FocusState {
    Focus,
    Unfocus,
}

#[derive(Event)]
pub struct ChatFocusStateChangeEvent {
    pub state: FocusState,
}
