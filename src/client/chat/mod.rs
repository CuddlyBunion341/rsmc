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
        app.add_systems(Update, systems::send_messages_system);
        app.add_systems(Update, systems::handle_events_system);
        app.add_systems(Update, systems::handle_chat_focus_system);
        app.add_systems(Update, systems::handle_chat_input_system);
        app.insert_resource(resources::ChatHistory::default());
        app.add_event::<events::ChatSyncEvent>();
        app.add_event::<events::SendMessageEvent>();
    }
}
