use crate::prelude::*;

#[derive(Event)]
pub struct ChunkMeshUpdateEvent {
    pub position: Vec3,
}

#[derive(Event)]
pub struct BlockUpdateEvent {
    pub position: Vec3,
    pub block: BlockId,
}
