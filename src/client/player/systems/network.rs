use crate::prelude::*;

pub fn broadcast_player_attributes_system(
    mut client: ResMut<RenetClient>,
    query: Query<(&player_components::Player, &Transform)>,
) {
    let (_, transform) = query.single();

    let player_state = lib::PlayerState {
        position: transform.translation,
        rotation: transform.rotation
    };

    client.send_message(
        DefaultChannel::ReliableUnordered,
        bincode::serialize(&lib::NetworkingMessage::PlayerUpdate(player_state)).unwrap(),
    );
}
