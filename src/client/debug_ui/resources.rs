use crate::prelude::*;

#[derive(Resource)]
pub struct DebugUi {
    pub is_visible: bool,
    pub position: Vec3,
    pub rotation: Vec3,
    pub selected_block: Vec3,
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
