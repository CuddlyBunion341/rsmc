use crate::prelude::*;

pub fn receive_message_system(
    mut server: ResMut<RenetServer>,
) {
    for client_id in server.clients_id() {
        let _message_bytes = server.receive_message(client_id, DefaultChannel::Unreliable);
        // TODO: handle message
        // let message = bincode::deserialize(&message_bytes.unwrap()).unwrap();
    }
}
