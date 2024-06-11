use bevy::{ecs::event::Event, math::Vec3};

use crate::blocks::BlockId;

#[derive(Event)]
pub struct BlockUpdateEvent {
    pub position: Vec3,
    pub block: BlockId,
}

#[derive(Event)]
pub struct ChunkMeshUpdateEvent {
    pub position: Vec3,
}

#[derive(Event)]
pub struct ColliderUpdateEvent {
    pub position: [f32; 3],
}
