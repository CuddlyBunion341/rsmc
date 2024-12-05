use crate::prelude::*;

#[derive(Component, Default, Debug, Clone)]
pub struct ExampleComponent {
    pub value: i32,
    pub name: String,
    pub active: bool,
}

#[derive(Component)]
pub struct ChatMessageContainer {
    pub enable_input: bool,
}

#[derive(Component)]
pub struct ChatMessageElement;
