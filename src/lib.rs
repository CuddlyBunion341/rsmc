use std::collections::HashMap;

use bevy::math::{Quat, Vec3};
use renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerState {
    pub position: Vec3,
    pub rotation: Quat,
}

#[derive(Serialize, Deserialize)]
pub enum NetworkingMessage {
    PlayerJoin(ClientId),
    PlayerLeave(ClientId),
    PlayerUpdate(PlayerState),
    PlayerSync(HashMap<ClientId, PlayerState>),
}

