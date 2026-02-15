use std::collections::HashSet;

use bevy::tasks::Task;

use crate::prelude::*;

#[derive(Resource)]
pub struct SpawnRegionLoaded(pub bool);

impl SpawnRegionLoaded {
    pub fn is_loaded(resource: Res<SpawnRegionLoaded>) -> bool {
        resource.0
    }
}

#[derive(Resource, Default)]
pub struct RequestedChunks {
    pub previous_chunks: HashSet<ChunkPosition>,
}

#[derive(Eq, Hash, Clone, PartialEq)]
pub enum MeshType {
    Solid,
    Transparent,
}

pub struct ChunkMeshes {
    pub cube_mesh: Option<Mesh>,
    pub cross_mesh: Option<Mesh>,
}

pub struct MeshTask(pub Task<ChunkMeshes>);
pub struct FutureChunkMesh {
    pub position: ChunkPosition,
    pub meshes_task: MeshTask,
}

#[derive(Resource, Default)]
pub struct MesherTasks {
    pub task_list: Vec<FutureChunkMesh>,
}

#[derive(Resource, Default)]
pub struct ChunkEntityMap {
    map: HashMap<ChunkPosition, Vec<Entity>>,
}

#[derive(Resource, Default)]
pub struct SpawnRegion {
    pub origin_chunk_position: ChunkPosition,
}

impl SpawnRegion {
    pub fn from_world_position(world_position: IVec3) -> Self {
        Self {
            origin_chunk_position: ChunkManager::world_position_to_chunk_position(world_position),
        }
    }
}

impl ChunkEntityMap {
    pub fn count(&self) -> usize {
        self.map.len()
    }

    pub fn add(&mut self, chunk_position: ChunkPosition, entity: Entity) {
        self.map.entry(chunk_position).or_default().push(entity);
    }

    pub fn remove(&mut self, chunk_position: ChunkPosition) -> Option<Vec<Entity>> {
        self.map.remove(&chunk_position)
    }

    pub fn extract_outside_distance(
        &mut self,
        origin: &ChunkPosition,
        distance: &IVec2,
    ) -> Vec<(ChunkPosition, Vec<Entity>)> {
        let extracted: HashMap<ChunkPosition, Vec<Entity>> = self
            .map
            .extract_if(|k, _v| {
                (k.x - origin.x).abs() > distance[0] || (k.z - origin.z).abs() > distance[1]
            })
            .collect();

        extracted.into_iter().collect()
    }
}

#[derive(Resource)]
pub struct RenderMaterials {
    pub transparent_material: Option<Handle<StandardMaterial>>,
    pub chunk_material: Option<Handle<StandardMaterial>>,
}

impl Default for RenderMaterials {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderMaterials {
    pub fn new() -> RenderMaterials {
        RenderMaterials {
            transparent_material: None,
            chunk_material: None,
        }
    }
}
