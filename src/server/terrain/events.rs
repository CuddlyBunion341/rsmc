use terrain_resources::TextureType;

use crate::prelude::*;

#[derive(Event)]
pub struct BlockUpdateEvent {
    pub position: Vec3,
    pub block: BlockId,
}

// visualizer
#[derive(Event)]
pub struct RegenerateHeightMapEvent(pub TextureType);

#[derive(Event)]
pub struct WorldRegenerateEvent;
