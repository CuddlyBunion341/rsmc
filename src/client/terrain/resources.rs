use terrain_util::client_block::MeshRepresentation;

use crate::prelude::*;

#[derive(Resource)]
pub struct SpawnAreaLoaded(pub bool);

impl SpawnAreaLoaded {
    pub fn is_loaded(resource: Res<SpawnAreaLoaded>) -> bool {
        resource.0
    }
}

#[derive(Resource)]
pub struct Mesher {
    pub mesh_handles: HashMap<MeshRepresentation, Handle<Mesh>>,
    pub transparent_material_handle: Option<Handle<StandardMaterial>>,
    pub chunk_material_handle: Option<Handle<StandardMaterial>>,
}

impl Default for Mesher {
    fn default() -> Self {
        Self::new()
    }
}

impl Mesher {
    pub fn new() -> Mesher {
        Mesher {
            mesh_handles: HashMap::new(),
            transparent_material_handle: None,
            chunk_material_handle: None,
        }
    }
}
