use bevy::{ecs::{component::Component, world::CommandQueue}, math::Vec3, prelude::Mesh, tasks::Task};

use super::util::GeometryData;

#[derive(Component)]
pub struct ChunkMesh {
    pub key: [i32; 3],
}

#[derive(Component)]
pub struct FutureChunk(pub Task<(Vec3, Option<Mesh>)>);
