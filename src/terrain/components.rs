use bevy::ecs::component::Component;

#[derive(Component)]
pub struct ChunkMesh {
    pub key: [i32; 3],
}

#[derive(Component)]
pub struct MyChunk {
    pub position: [i32; 3],
}

