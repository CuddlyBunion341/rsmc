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
        app.insert_resource(resources::ExampleResource::default());
      }
    }
