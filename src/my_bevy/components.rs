use bevy::ecs::component::Component;

#[derive(Component)]
pub struct ChunkMesh {
    pub key: [i32; 3],
}

#[derive(Component)]
pub struct MyCube;

#[derive(Component)]
pub struct MyChunk {
    pub position: [i32; 3],
}

#[derive(Component)]
pub struct HighlightCube;

#[derive(Component)]
pub struct MyCollider {
    pub key: u32,
}

