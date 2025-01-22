// std crates
pub use std::collections::HashMap;
pub use std::net::UdpSocket;
pub use std::time::SystemTime;

// bevy crates
pub use bevy::app::{App, Plugin, Startup, Update};
pub use bevy::ecs::event::*;
pub use bevy::ecs::system::*;
pub use bevy::log::{debug, error, info, warn};
pub use bevy::math::*;
pub use bevy::MinimalPlugins;

// networking crates
pub use bevy_renet::netcode::NetcodeServerPlugin;
pub use bevy_renet::netcode::NetcodeServerTransport;
pub use bevy_renet::netcode::ServerAuthentication;
pub use bevy_renet::netcode::ServerConfig;
pub use bevy_renet::RenetServerPlugin;
pub use renet::DefaultChannel;
pub use renet::*;

pub use self::lib::Chunk;
pub use self::terrain_util::Block;
pub use noise::NoiseFn;
pub use noise::Perlin;

// my crates
pub use rsmc as lib;
pub use rsmc::BlockId;

pub use crate::networking::systems as networking_systems;

pub use crate::player::resources as player_resources;
pub use crate::player::systems as player_systems;

pub use crate::terrain::events as terrain_events;
pub use crate::terrain::resources as terrain_resources;
pub use crate::terrain::systems as terrain_systems;
pub use crate::terrain::util as terrain_util;

pub use crate::chat::events as chat_events;
pub use crate::chat::resources as chat_resources;
pub use crate::chat::systems as chat_systems;
