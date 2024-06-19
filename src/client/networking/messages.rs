use crate::prelude::*;

use self::remote_player_events::PlayerState;

#[derive(Serialize, Deserialize)]
pub enum NetworkingMessage {
    PlayerJoin(ClientId),
    PlayerLeave(ClientId),
    PlayerSync(HashMap<ClientId, PlayerState>),
}
