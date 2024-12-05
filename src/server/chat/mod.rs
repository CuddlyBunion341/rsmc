use crate::prelude::*;

pub mod resources;
pub mod systems;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        info!("Building ChatPlugin");
        app.insert_resource(resources::ChatHistory::new());
    }
}
