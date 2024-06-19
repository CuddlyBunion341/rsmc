use serde::{Deserialize, Serialize};

use crate::prelude::*;

use self::player_resources::PlayerState;

#[derive(Serialize, Deserialize)]
pub enum NetworkingMessage {
    PlayerJoin(ClientId),
    PlayerLeave(ClientId),
    PlayerUpdate(PlayerState),
    PlayerSync(HashMap<ClientId, PlayerState>),
}

