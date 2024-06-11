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

#[derive(Resource)]
pub struct ChunkManager {
    pub chunks: HashMap<[i32; 3], Chunk>,
}

