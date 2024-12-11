use crate::prelude::*;

pub mod resources;
pub mod systems;
pub mod events;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        info!("Building ChatPlugin");
        app.insert_resource(resources::ChatHistory::new());
        app.add_systems(Update, chat_systems::handle_network_chat_message_send);
        app.add_event::<chat_events::PlayerChatMessageSendEvent>();
    }
}
