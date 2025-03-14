use bevy::ecs::component::Component;

#[derive(Component)]
pub struct ChunkMesh {
    pub key: [i32; 3],
}
