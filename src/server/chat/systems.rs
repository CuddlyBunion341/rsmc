use crate::prelude::*;

pub fn broadcast_messages(
    mut server: ResMut<RenetServer>,
    messages: Res<chat_resources::ChatHistory>
) {
}
