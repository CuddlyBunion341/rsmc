use crate::prelude::*;

pub fn setup_world_system(
    mut commands: Commands,
    mut chunk_manager: ResMut<terrain_resources::ChunkManager>,
) {
    let generator = terrain_util::generator::Generator::new(0);

    let render_distance = 16;

    let mut chunks = terrain_resources::ChunkManager::instantiate_chunks(
        Vec3::new(0.0, 0.0, 0.0),
        render_distance,
    );

    for chunk in &mut chunks {
        generator.generate_chunk(chunk);
    }

    chunk_manager.insert_chunks(chunks);
}
