use crate::prelude::*;

#[derive(Event)]
pub struct ColliderUpdateEvent {
    pub position: [f32; 3],
}
