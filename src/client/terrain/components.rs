use bevy::ecs::component::Component;

use super::resources::MeshType;

#[derive(Component)]
pub struct ChunkMesh {
    pub key: [i32; 3],
    pub mesh_type: MeshType,
}
