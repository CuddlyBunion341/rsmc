use crate::prelude::*;

#[derive(Resource)]
pub struct DebugUi {
    is_visible: bool,
    position: Vec3,
    rotation: Vec3,
    selected_block: Vec3,
}

impl DebugUi {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            selected_block: Vec3::ZERO,
        }
    }
}
