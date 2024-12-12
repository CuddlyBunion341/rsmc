use crate::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct ChatHistory {
    pub messages: Vec<lib::ChatMessage>,
}

#[derive(Resource, Default)]
pub struct ChatState {
    pub just_focused: bool,
}
