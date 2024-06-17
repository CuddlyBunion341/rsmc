use bevy::{ecs::system::Resource, math::Vec3};

#[derive(Resource)]
pub struct BlockSelection {
    pub position: Option<Vec3>,
    pub normal: Option<Vec3>,
}

impl BlockSelection {
    pub fn new() -> Self {
        Self {
            position: None,
            normal: None,
        }
    }
}

#[derive(Resource)]
pub struct LastPlayerPosition(pub Vec3);

impl LastPlayerPosition {
    pub fn new() -> Self {
        Self(Vec3::ZERO)
    }
}
