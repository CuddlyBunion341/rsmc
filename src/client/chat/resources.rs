use crate::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct ChatHistory {
    pub messages: Vec<lib::ChatMessage>
}
