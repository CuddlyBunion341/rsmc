use std::collections::HashMap;

use bevy::{
    math::{Quat, Vec3},
    prelude::Resource,
};
use chrono::DateTime;
use renet::ClientId;
use serde::{Deserialize, Serialize};

pub const SERVER_MESSAGE_ID: ClientId = 0;

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

        let client_name = match self.client_id {
            SERVER_MESSAGE_ID => "SERVER".to_string(),
            _ => self.client_id.to_string(),
        };

        format!("[{}] {}: {}", timestamp_string, client_name, self.message)
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
    SingleChatMessageSync(ChatMessage),
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

#[derive(Resource)]
pub struct ChunkManager {
    pub chunks: HashMap<[i32; 3], Chunk>,
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkManager {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn instantiate_chunks_vec(position: Vec3, render_distance: Vec3) -> Vec<Chunk> {
        let render_distance_x = render_distance.x as i32;
        let render_distance_y = render_distance.y as i32;
        let render_distance_z = render_distance.z as i32;

        let mut chunks: Vec<Chunk> = Vec::new();

        for x in -render_distance_x..render_distance_x {
            for y in -render_distance_y..render_distance_y {
                for z in -render_distance_z..render_distance_z {
                    let chunk_position = Vec3::new(
                        x as f32 + position.x,
                        y as f32 + position.y,
                        z as f32 + position.z,
                    );
                    let chunk = Chunk::new(chunk_position);
                    chunks.push(chunk);
                }
            }
        }

        chunks
    }

    pub fn instantiate_chunks(position: Vec3, render_distance: i32) -> Vec<Chunk> {
        Self::instantiate_chunks_vec(
            position,
            Vec3::new(
                render_distance as f32,
                render_distance as f32,
                render_distance as f32,
            ),
        )
    }

    pub fn instantiate_new_chunks(&mut self, position: Vec3, render_distance: i32) -> Vec<Chunk> {
        let chunks = Self::instantiate_chunks(position, render_distance);

        chunks
            .into_iter()
            .filter(|chunk| {
                let chunk_position = chunk.position;
                let chunk = self.get_chunk_mut(chunk_position);
                chunk.is_none()
            })
            .collect()
    }

    pub fn insert_chunk(&mut self, chunk: Chunk) {
        self.chunks
            .insert(Self::position_to_key(chunk.position), chunk);
    }

    pub fn insert_chunks(&mut self, chunks: Vec<Chunk>) {
        for chunk in chunks {
            self.insert_chunk(chunk);
        }
    }

    pub fn position_to_key(position: Vec3) -> [i32; 3] {
        [position.x as i32, position.y as i32, position.z as i32]
    }

    pub fn set_chunk(&mut self, position: Vec3, chunk: Chunk) {
        let Vec3 { x, y, z } = position;

        self.chunks.insert([x as i32, y as i32, z as i32], chunk);
    }

    pub fn get_chunk(&self, position: Vec3) -> Option<&Chunk> {
        let Vec3 { x, y, z } = position.floor();

        self.chunks.get(&[x as i32, y as i32, z as i32])
    }

    pub fn get_chunk_mut(&mut self, position: Vec3) -> Option<&mut Chunk> {
        let Vec3 { x, y, z } = position.floor();

        self.chunks.get_mut(&[x as i32, y as i32, z as i32])
    }

    pub fn set_block(&mut self, position: Vec3, block: BlockId) {
        match self.chunk_from_selection(position) {
            Some(chunk) => {
                let chunk_position = Vec3::new(
                    chunk.position[0] * CHUNK_SIZE as f32,
                    chunk.position[1] * CHUNK_SIZE as f32,
                    chunk.position[2] * CHUNK_SIZE as f32,
                );
                let local_position = (position - chunk_position).floor();
                chunk.set(
                    local_position.x as usize,
                    local_position.y as usize,
                    local_position.z as usize,
                    block,
                );
            }
            None => {
                println!("No chunk found");
            }
        }
    }

    pub fn get_block(&mut self, position: Vec3) -> Option<BlockId> {
        match self.chunk_from_selection(position) {
            Some(chunk) => {
                let chunk_position = Vec3::new(
                    chunk.position[0] * CHUNK_SIZE as f32,
                    chunk.position[1] * CHUNK_SIZE as f32,
                    chunk.position[2] * CHUNK_SIZE as f32,
                );
                let local_position = (position - chunk_position).floor();
                Some(chunk.get(
                    local_position.x as usize,
                    local_position.y as usize,
                    local_position.z as usize,
                ))
            }
            None => {
                // println!("No chunk found for block at {:?}", position);
                None
            }
        }
    }

    fn chunk_from_selection(&mut self, position: Vec3) -> Option<&mut Chunk> {
        let chunk_position = position / CHUNK_SIZE as f32;
        self.get_chunk_mut(chunk_position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_manager_new() {
        let chunk_manager = ChunkManager::new();
        assert!(chunk_manager.chunks.is_empty());
    }

    #[test]
    fn test_instantiate_chunks() {
        let position = Vec3::new(0.0, 0.0, 0.0);

        let width = 2;
        let height = 3;
        let depth = 4;

        let render_distance = Vec3::new(width as f32, height as f32, depth as f32);

        let chunks = ChunkManager::instantiate_chunks_vec(position, render_distance);
        assert_eq!(chunks.len(), (2 * width * 2 * height * 2 * depth) as usize,);
    }

    #[test]
    fn test_insert_chunks() {
        let mut chunk_manager = ChunkManager::new();
        let position = Vec3::new(0.0, 0.0, 0.0);
        let render_distance = 2;
        let chunks = ChunkManager::instantiate_chunks(position, render_distance);

        let render_diameter = render_distance * 2;

        chunk_manager.insert_chunks(chunks);
        assert_eq!(
            chunk_manager.chunks.len(),
            (render_diameter * render_diameter * render_diameter) as usize
        );
    }

    #[test]
    fn test_set_and_get_chunk_mut() {
        let mut chunk_manager = ChunkManager::new();
        let position = Vec3::new(0.0, 0.0, 0.0);
        let chunk = Chunk::new(position);

        chunk_manager.set_chunk(position, chunk);
        let retrieved_chunk = chunk_manager.get_chunk_mut(position).unwrap();
        assert_eq!(retrieved_chunk.position, chunk.position);
    }

    #[test]
    fn test_set_and_get_block() {
        let mut chunk_manager = ChunkManager::new();
        let position = Vec3::new(0.0, 0.0, 0.0);
        let chunk = Chunk::new(position);

        chunk_manager.set_chunk(position, chunk);
        let block_position = Vec3::new(1.0, 1.0, 1.0);
        let block_id = BlockId::Stone;

        chunk_manager.set_block(block_position, block_id);
        let retrieved_block = chunk_manager.get_block(block_position).unwrap();
        assert_eq!(retrieved_block, block_id);
    }
}
