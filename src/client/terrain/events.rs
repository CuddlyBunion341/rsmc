use crate::prelude::*;

#[derive(Event)]
pub struct SpawnAreaReadyEvent;

#[derive(Event)]
pub struct ChunkMeshUpdateEvent {
    pub position: Vec3,
}

#[derive(Event)]
pub struct BlockUpdateEvent {
    pub position: Vec3,
    pub block: BlockId,
    pub from_network: bool,
}
