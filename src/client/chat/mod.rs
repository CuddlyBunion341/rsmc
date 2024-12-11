use crate::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        info!("Building ChatPlugin");

        app.add_systems(Startup, systems::setup_chat_container);
        app.add_systems(
            Update,
            (
                systems::handle_chat_input_system,
                systems::send_messages_system,
                systems::handle_chat_focus_input_event,
                systems::handle_window_focus_events,
                systems::handle_chat_focus_player_events,
                systems::handle_chat_container_focus_events,
                systems::handle_chat_input_focus_events,
                systems::handle_events_system,
                systems::add_message_to_chat_container_system,
            ),
        );
        app.insert_resource(resources::ChatHistory::default());
        app.add_event::<events::ChatSyncEvent>();
        app.add_event::<events::FocusChangeEvent>();
        app.add_event::<events::SendMessageEvent>();
        app.add_event::<events::SingleChatSendEvent>();
    }
}
