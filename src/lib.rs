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
    ChunkRequest { position: Vec3, client_id: ClientId },
    BlockUpdate { position: Vec3, block: BlockId },
}

macro_rules! enum_from_u8 {
    ($name:ident { $( $variant:ident ),* $(,)? }) => {
        #[repr(u8)]
        #[derive(Debug, PartialEq, Copy, Clone, Deserialize, Serialize)]
        pub enum $name {
            $( $variant ),*
        }

        impl $name {
            pub fn from_u8(value: u8) -> Option<$name> {
                match value {
                    $(x if x == $name::$variant as u8 => Some($name::$variant),)*
                        _ => None,
                }
            }

            pub fn to_u8(&self) -> u8 {
                self.clone() as u8
            }
        }
    };
}

enum_from_u8! {
    BlockId {
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
}
