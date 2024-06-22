use terrain_events::BlockUpdateEvent;

use crate::prelude::*;

pub struct ClientsWithBlockUpdate {
    pub clients: Vec<ClientId>,
    pub block_update: BlockUpdateEvent,
}

#[derive(Resource)]
pub struct BlockUpdateResource {
    pub updates: Vec<ClientsWithBlockUpdate>,
}

impl BlockUpdateResource {
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
        }
    }
}
