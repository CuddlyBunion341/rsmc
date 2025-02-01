use crate::prelude::*;

pub fn setup_world_system(
    mut chunk_manager: ResMut<ChunkManager>,
    generator: Res<terrain_resources::Generator>,
) {
    let render_distance = Vec3::new(6.0, 2.0, 6.0);

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
    use bevy::{
        log::info,
        math::{Vec2, Vec3},
        prelude::{EventReader, EventWriter, ResMut},
    };
    use bevy_inspector_egui::{
        bevy_egui::EguiContexts,
        egui::{self, Color32, ColorImage, ImageData, TextureOptions},
    };
    use rayon::iter::IntoParallelIterator;

    use rayon::iter::ParallelIterator;
    use renet::{DefaultChannel, RenetServer};
    use rsmc::{Chunk, ChunkManager, NetworkingMessage};

    use super::{terrain_events, terrain_resources};

    fn generate_terrain_heightmap(
        generator: &terrain_resources::Generator,
        origin: Vec3,
        size: Vec3,
    ) -> ImageData {
        let mut data = vec![0; (size.x * size.z) as usize];

        let width = size.x as usize;
        let height = size.z as usize;

        for x in 0..width {
            for z in 0..height {
                let sample_position =
                    Vec2::new((origin.x + x as f32) / 1.0, (origin.z + z as f32) / 1.0);
                let value = generator.sample_2d(sample_position, &generator.params.height_params);
                let value = value * size.y as f64;
                let value = value as u8;
                data[x + z * width] = value;
            }
        }

        let color_data: Vec<Color32> = data
            .iter()
            .map(|&value| Color32::from_gray(value))
            .collect();

        let color_image: ColorImage = ColorImage {
            size: [width, height],
            pixels: color_data,
        };

        ImageData::Color(color_image.into())
    }

    pub fn handle_regenerate_event_system(
        mut events: EventReader<terrain_events::WorldRegenerateEvent>,
        mut chunk_manager: ResMut<ChunkManager>,
        generator: ResMut<terrain_resources::Generator>,
        mut server: ResMut<RenetServer>,
    ) {
        for _ in events.read() {
            info!("Regenerating world");
            let existing_chunk_positions = chunk_manager.get_all_chunk_positions();

            let new_chunks: Vec<Chunk> = existing_chunk_positions
                .into_par_iter()
                .map(|chunk_position| {
                    let mut chunk = Chunk::new(chunk_position);
                    info!("Generating chunk at {:?}", chunk_position);
                    generator.generate_chunk(&mut chunk);
                    chunk
                })
                .collect();

            new_chunks.into_iter().for_each(|chunk| {
                chunk_manager.insert_chunk(chunk);
            });

            info!("Successfully regenerated world");
            info!("Sending chunk requests for all chunks");

            server.broadcast_message(
                DefaultChannel::ReliableUnordered,
                bincode::serialize(
                    &NetworkingMessage::ServerAsksClientNicelyToRerequestChunkBatch(),
                )
                .unwrap(),
            );
        }
    }

    pub fn regenerate_heightmap_system(
        mut events: EventReader<terrain_events::RegenerateHeightMapEvent>,
        generator: ResMut<terrain_resources::Generator>,
        mut noise_texture: ResMut<terrain_resources::NoiseTexture>,
        mut contexts: EguiContexts,
    ) {
        for _ in events.read() {
            let width = 512;
            let height = 512;
            let depth = 512;

            let image_data = generate_terrain_heightmap(
                &generator,
                Vec3::ZERO,
                Vec3::new(width as f32, height as f32, depth as f32),
            );

            noise_texture.texture = Some(contexts.ctx_mut().load_texture(
                "terrain-texture",
                image_data,
                TextureOptions::default(),
            ));
            noise_texture.size = Vec2::new(width as f32, height as f32);
        }
    }

    pub fn prepare_visualizer_texture_system(
        mut event_writer: EventWriter<terrain_events::RegenerateHeightMapEvent>,
    ) {
        event_writer.send(terrain_events::RegenerateHeightMapEvent);
    }

    macro_rules! add_slider {
        ($ui:expr, $changed:expr, $value:expr, $range:expr, $text:expr) => {{
            $changed |= $ui
                .add(egui::widgets::Slider::new($value, $range).text($text))
                .changed();
        }};
    }

    #[rustfmt::skip]
    pub fn render_visualizer_system(
        mut contexts: EguiContexts,
        noise_texture: ResMut<terrain_resources::NoiseTexture>,
        mut generator: ResMut<terrain_resources::Generator>,
        mut event_writer: EventWriter<terrain_events::RegenerateHeightMapEvent>,
        mut world_regenerate_event_writer: EventWriter<terrain_events::WorldRegenerateEvent>,
    ) {
        if let Some(texture_handle) = &noise_texture.texture {
            egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
                ui.label("world");

                let mut changed = false;

                add_slider!(ui, changed, &mut generator.params.height_params.octaves, 1..=8, "octaves");
                add_slider!(ui, changed, &mut generator.params.height_params.height, 0.0..=10.0, "height");
                add_slider!(ui, changed, &mut generator.params.height_params.lacuranity, 0.0..=4.0, "lacuranity");
                add_slider!(ui, changed, &mut generator.params.height_params.frequency, 0.0..=1.0, "frequency");
                add_slider!(ui, changed, &mut generator.params.height_params.amplitude, 0.0..=20.0, "amplitude");
                add_slider!(ui, changed, &mut generator.params.height_params.persistence, 0.0..=1.0, "persistence");

                if changed {
                    event_writer.send(terrain_events::RegenerateHeightMapEvent);
                };

                ui.label(format!("{:?}", generator.params.height_params));

                if ui.button("Regenerate world").clicked() {
                    world_regenerate_event_writer.send(terrain_events::WorldRegenerateEvent);
                }

                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    texture_handle.id(),
                    texture_handle.size_vec2(),
                )));
            });
        }
    }
}
