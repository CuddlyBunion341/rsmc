use crate::prelude::*;

#[derive(Resource)]
pub struct BlockSelection {
    pub position: Option<Vec3>,
    pub normal: Option<Vec3>,
}

#[derive(Resource)]
pub struct PlayerSpawned(pub bool);

impl PlayerSpawned {
    pub fn is_spawned(resource: Res<PlayerSpawned>) -> bool {
        resource.0
    } 
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

impl Default for LastPlayerPosition {
    fn default() -> Self {
        Self::new()
    }
}

impl LastPlayerPosition {
    pub fn new() -> Self {
        Self(Vec3::ZERO)
    }
}
