use crate::prelude::*;

#[derive(Component)]
pub struct ChatMessageContainer {
    pub focused: bool,
}

#[derive(Component)]
pub struct ChatMessageInputElement {
    pub focused: bool,
}

#[derive(Component)]
pub struct ChatMessageElement;
