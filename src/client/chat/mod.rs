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
                systems::process_chat_input_system,
                systems::handle_chat_focus_player_controller_events,
                systems::handle_window_focus_events,
                systems::focus_chat_input_system,
                systems::send_messages_system,
                systems::handle_focus_events,
                systems::handle_chat_message_sync_event,
                systems::add_message_to_chat_container_system,
            ),
        );
        app.insert_resource(resources::ChatHistory::default());
        app.insert_resource(resources::ChatState::default());
        app.add_event::<events::ChatSyncEvent>();
        app.add_event::<events::ChatFocusStateChangeEvent>();
        app.add_event::<events::ChatMessageSendEvent>();
        app.add_event::<events::SingleChatSendEvent>();
    }
}
