use renet::DefaultChannel;

use crate::prelude::*;

pub fn receive_message_system(
    mut client: ResMut<RenetClient>,
) {
    while let Some(_message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        // TODO: process message
        // let message = bincode::deserialize(&message).unwrap();
    }

    while let Some(_message) = client.receive_message(DefaultChannel::Unreliable) {
        // TODO: process message
        // let message = bincode::deserialize(&message).unwrap();
    }
}

