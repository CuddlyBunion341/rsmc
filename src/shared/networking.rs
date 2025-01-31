use std::time::Duration;

use bevy::{math::{Quat, Vec3}, prelude::Resource};
use chrono::DateTime;
use renet::{ChannelConfig, ClientId, ConnectionConfig, SendType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{BlockId, Chunk};

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
    UpdateGeneratorParams(TerrainGeneratorParams),
}

#[derive(Serialize, Deserialize, Debug, Resource, Copy, Clone)]
pub struct TerrainGeneratorParams {
    pub height_params: NoiseFunctionParams,
    pub density_params: NoiseFunctionParams,
}

impl Default for TerrainGeneratorParams {
    fn default() -> Self {
        Self {
            height_params: NoiseFunctionParams {
                octaves: 4,
                height: 0.0,
                lacuranity: 2.0,
                frequency: 1.0 / 60.0,
                amplitude: 10.0,
                persistence: 0.5,
            },
            density_params: NoiseFunctionParams {
                octaves: 4,
                height: 0.0,
                lacuranity: 2.0,
                frequency: 1.0 / 60.0,
                amplitude: 10.0,
                persistence: 0.5,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct NoiseFunctionParams {
    pub octaves: i32,
    pub height: f64,
    pub lacuranity: f64,
    pub frequency: f64,
    pub amplitude: f64,
    pub persistence: f64,
}

const CHANNELS: [ChannelConfig; 3] = [
    ChannelConfig {
        channel_id: 0,
        max_memory_usage_bytes: 1024 * 1024 * 1024 * 1024,
        send_type: SendType::Unreliable,
    },
    ChannelConfig {
        channel_id: 1,
        max_memory_usage_bytes: 1024 * 1024 * 1024 * 1024,
        send_type: SendType::ReliableOrdered {
            resend_time: Duration::from_millis(300),
        },
    },
    ChannelConfig {
        channel_id: 2,
        max_memory_usage_bytes: 1024 * 1024 * 1024 * 1024,
        send_type: SendType::ReliableUnordered {
            resend_time: Duration::from_millis(300),
        },
    },
];

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        client_channels_config: CHANNELS.to_vec(),
        ..Default::default()
    }
}
