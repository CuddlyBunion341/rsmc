use crate::prelude::*;

#[derive(Resource)]
pub struct DebugUi {
    entity: Entity,
    is_visible: bool,
    position: Vec3,
    rotation: Vec3,
    selected_block: Vec3,
}

impl DebugUi {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            is_visible: false,
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            selected_block: Vec3::ZERO,
        }
    }
}
