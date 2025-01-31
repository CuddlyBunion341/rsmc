use crate::prelude::*;

pub fn setup_world_system(
    mut chunk_manager: ResMut<ChunkManager>,
    generator: Res<terrain_resources::Generator>,
) {
    let render_distance = Vec3::new(12.0, 2.0, 12.0);

    info!("Generating chunks");

    let mut chunks = ChunkManager::instantiate_chunks(Vec3::ZERO, render_distance);

    chunks.par_iter_mut().for_each(|chunk| {
        info!("Generating chunk at {:?}", chunk.position);
        generator.generate_chunk(chunk);
    });

    chunk_manager.insert_chunks(chunks);
}

pub use visualizer::*;

mod visualizer {

    use bevy_inspector_egui::{bevy_egui::EguiContexts, egui};

    pub fn render_visualizer_system(mut contexts: EguiContexts) {
        egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
            ui.label("world");
        });
    }
}
