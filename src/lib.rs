use std::collections::HashMap;

use bevy::math::{Quat, Vec3};
use renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerState {
    pub position: Vec3,
    pub rotation: Quat,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkingMessage {
    PlayerJoin(ClientId),
    PlayerLeave(ClientId),
    PlayerUpdate(PlayerState),
    PlayerSync(HashMap<ClientId, PlayerState>),
    BlockUpdate { position: Vec3, block: BlockId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum BlockId {
    Air,
    Grass,
    Dirt,
    Stone,
    Bedrock,
    RedSand,
    BrownTerracotta,
    CyanTerracotta,
    GrayTerracotta,
    LightGrayTerracotta,
    OrangeTerracotta,
    RedTerracotta,
    Terracotta,
    YellowTerracotta,
}
