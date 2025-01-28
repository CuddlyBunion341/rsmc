use crate::prelude::*;

pub fn setup_world_system(mut chunk_manager: ResMut<ChunkManager>) {
    let generator = terrain_util::generator::Generator::new(0);

    let render_distance = Vec3::new(12.0, 2.0, 12.0);

    info!("Generating chunks");

    let mut chunks = ChunkManager::instantiate_chunks(Vec3::ZERO, render_distance);

    chunks.par_iter_mut().for_each(|chunk| {
        info!("Generating chunk at {:?}", chunk.position);
        generator.generate_chunk(chunk);
    });

    chunk_manager.insert_chunks(chunks);
}
