use crate::prelude::*;

pub fn setup_world_system(mut chunk_manager: ResMut<terrain_resources::ChunkManager>) {
    let generator = terrain_util::generator::Generator::new(0);

    let render_distance = 4;

    info!("Generating chunks");

    let mut chunks = terrain_resources::ChunkManager::instantiate_chunks(
        Vec3::new(0.0, 0.0, 0.0),
        render_distance,
    );

    for chunk in &mut chunks {
        info!("Generating chunk at {:?}", chunk.position);
        generator.generate_chunk(chunk);
    }

    chunk_manager.insert_chunks(chunks);
}
