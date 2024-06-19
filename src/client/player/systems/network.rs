use crate::prelude::*;

pub fn broadcast_player_attributes_system(
    mut client: ResMut<RenetClient>,
    query: Query<(&player_components::Player, &Transform)>,
) {
    let (player, transform) = query.single();
}
