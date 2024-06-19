use crate::{networking::messages::NetworkingMessage, prelude::*};

pub fn broadcast_player_attributes_system(
    mut server: ResMut<RenetServer>,
    player_states: Res<player_resources::PlayerStates>,
) {
    for client_id in server.clients_id() {
        let mut other_player_states = player_states.players.clone();
        other_player_states.remove(&client_id);

        server.send_message(
            client_id,
            DefaultChannel::ReliableUnordered,
            bincode::serialize(&NetworkingMessage::PlayerSync(other_player_states)).unwrap(),
        );
    }
}
