use crate::prelude::*;

pub fn debug_system(mut debug_ui: ResMut<debug_ui_resources::DebugUi>, player_query: Query<(&player_components::Player, &Transform)>) {
let player_position= player_query.single();

match player_query.single() {
    Some(_, transform) => {

    }
}


   debug_ui.position = player_position;
   debug_ui.selected_block = Vec3::ZERO;
   debug_ui.rotation = Vec3::ZERO;
}
