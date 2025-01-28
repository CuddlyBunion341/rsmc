use crate::prelude::*;

#[derive(Event)]
pub struct BlockUpdateEvent {
    pub position: Vec3,
    pub block: lib::BlockId,
}
