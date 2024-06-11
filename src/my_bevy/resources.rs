use std::collections::HashMap;

use bevy::{ecs::system::Resource, math::Vec3, prelude::{Deref, DerefMut}};

use crate::chunk::Chunk;

#[derive(Resource, Deref, DerefMut)]
pub struct SelectedPosition(pub Option<Vec3>);

#[derive(Resource)]
pub struct BlockSelection {
    pub position: Option<Vec3>,
    pub normal: Option<Vec3>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SelectedNormal(pub Option<Vec3>);

#[derive(Resource)]
pub struct ChunkManager {
    pub chunks: HashMap<[i32; 3], Chunk>,
}

#[derive(Resource)]
pub struct LastPlayerPosition(pub Vec3);
