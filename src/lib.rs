use std::collections::HashMap;

use bevy::math::{Quat, Vec3};
use chrono::DateTime;
use renet::ClientId;
use serde::{Deserialize, Serialize};

pub const SERVER_MESSAGE_ID: ClientId = ClientId::from_raw(0);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerState {
    pub position: Vec3,
    pub rotation: Quat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub client_id: ClientId,
    pub message_id: usize,
    pub timestamp: i64,
    pub message: String,
}

impl ChatMessage {
    pub fn format_string(&self) -> String {
        let dt = DateTime::from_timestamp_millis(self.timestamp).expect("invalid timestamp");
        let timestamp_string = dt.to_string();
        format!(
            "[{}] {}: {}",
            timestamp_string, self.client_id, self.message
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkingMessage {
    PlayerJoin(ClientId),
    PlayerLeave(ClientId),
    PlayerUpdate(PlayerState),
    PlayerSync(HashMap<ClientId, PlayerState>),
    ChunkBatchRequest(Vec<Vec3>),
    ChunkBatchResponse(Vec<Chunk>),
    ChatMessageSend(String),
    ChatMessageSync(Vec<ChatMessage>),
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

use serde_big_array::BigArray;

pub const CHUNK_SIZE: usize = 32;
pub const PADDED_CHUNK_SIZE: usize = CHUNK_SIZE + 2;
pub const PADDED_CHUNK_USIZE: usize = PADDED_CHUNK_SIZE;
pub const CHUNK_LENGTH: usize = PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Chunk {
    #[serde(with = "BigArray")]
    pub data: [BlockId; CHUNK_LENGTH],
    pub position: Vec3,
}

impl Chunk {
    pub fn new(position: Vec3) -> Self {
        Self {
            data: [BlockId::Air; CHUNK_LENGTH],
            position,
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> BlockId {
        self.get_unpadded(x + 1, y + 1, z + 1)
    }

    pub fn get_unpadded(&self, x: usize, y: usize, z: usize) -> BlockId {
        self.data[Self::index(x, y, z)]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: BlockId) {
        self.set_unpadded(x + 1, y + 1, z + 1, value);
    }

    pub fn set_unpadded(&mut self, x: usize, y: usize, z: usize, value: BlockId) {
        self.data[Self::index(x, y, z)] = value;
    }

    #[rustfmt::skip]
    pub fn index(x: usize, y: usize, z: usize) -> usize {
        if (x >= PADDED_CHUNK_SIZE) || (y >= PADDED_CHUNK_SIZE) || (z >= PADDED_CHUNK_SIZE) {
            panic!("Index out of bounds: ({}, {}, {})", x, y, z);
        }
        x + PADDED_CHUNK_USIZE * (y + PADDED_CHUNK_USIZE * z)
    }

    pub fn key_eq_pos(key: [i32; 3], position: Vec3) -> bool {
        position.x as i32 == key[0] && position.y as i32 == key[1] && position.z as i32 == key[2]
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}
