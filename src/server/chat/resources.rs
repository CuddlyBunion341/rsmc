use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct ChatHistory {
    pub messages: Vec<ChatMessage>
}

impl ChatHistory {
    pub fn new() -> Self {
        Self {
            messages: Vec::new()
        }
    }
}

impl Default for ChatHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Resource, Default, Debug)]
pub struct ChatMessage {
    pub message_id: i32,
    pub client_id: usize,
    pub timestamp: u32,
    pub message: String,
}
