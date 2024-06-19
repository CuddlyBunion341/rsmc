use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub enum NetworkingMessage {
    PlayerJoin(ClientId),
    PlayerLeave(ClientId),
    PlayerSync(HashMap<ClientId, RemotePlayer>),
}
