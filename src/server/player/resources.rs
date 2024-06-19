use bevy::math::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerState {
    pub position: Vec3,
    pub rotation: Quat,
}

#[derive(Resource)]
pub struct PlayerStates {
    pub players: HashMap<ClientId, PlayerState>,
}

impl PlayerStates {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }
}

impl Default for PlayerStates {
    fn default() -> Self {
        Self::new()
    }
}
