use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct ChatHistory {
    pub messages: Vec<lib::ChatMessage>
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
