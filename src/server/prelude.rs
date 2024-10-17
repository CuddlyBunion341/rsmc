// my crates
pub use rsmc as lib;
pub use rsmc::BlockId;

pub use crate::networking::systems as networking_systems;

pub use crate::player::resources as player_resources;
pub use crate::player::systems as player_systems;

pub use crate::terrain::events as terrain_events;
pub use crate::terrain::resources as terrain_resources;
pub use crate::terrain::systems as terrain_systems;

// std crates
pub use std::collections::HashMap;
pub use std::net::UdpSocket;
pub use std::time::SystemTime;

// bevy crates
pub use bevy::app::{App, Plugin, Startup, Update};
pub use bevy::ecs::event::*;
pub use bevy::ecs::system::*;
pub use bevy::math::*;
pub use bevy::MinimalPlugins;
pub use bevy_log::{debug, error, info, warn};

// networking crates
pub use bevy_renet::transport::*;
pub use bevy_renet::RenetServerPlugin;
pub use renet::transport::*;
pub use renet::DefaultChannel;
pub use renet::*;
