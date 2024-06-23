use terrain_events::BlockUpdateEvent;

use crate::prelude::*;

#[derive(Resource)]
pub struct PastBlockUpdates {
    pub updates: Vec<BlockUpdateEvent>,
}

impl Default for PastBlockUpdates {
    fn default() -> Self {
        Self::new()
    }
}

impl PastBlockUpdates {
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
        }
    }
}
