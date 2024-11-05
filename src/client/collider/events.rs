use crate::prelude::*;

#[derive(Event)]
pub struct ColliderUpdateEvent {
    pub grid_center_position: [f32; 3],
}
