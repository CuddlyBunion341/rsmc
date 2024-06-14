use bevy::{ecs::system::Resource, math::Vec3};

#[derive(Resource)]
pub struct BlockSelection {
    pub position: Option<Vec3>,
    pub normal: Option<Vec3>,
}

#[derive(Resource)]
pub struct LastPlayerPosition(pub Vec3);
