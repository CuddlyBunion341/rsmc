use crate::prelude::*;

#[derive(Component)]
pub struct ChatMessageContainer {
    pub focused: bool
}


#[derive(Component)]
pub struct ChatMessageInputElement {
    pub enable_input: bool,
}

#[derive(Component)]
pub struct ChatMessageElement;
