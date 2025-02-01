use crate::prelude::*;

pub fn setup_world_system(
    mut chunk_manager: ResMut<ChunkManager>,
    generator: Res<terrain_resources::Generator>,
) {
    let render_distance = Vec3::new(12.0, 2.0, 12.0);

    return;

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
        asset::RenderAssetUsages,
        image::Image,
        log::info,
        math::{Vec2, Vec3},
        prelude::{EventReader, EventWriter, Res, ResMut},
        render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    };
    use bevy_inspector_egui::{
        bevy_egui::EguiContexts,
        egui::{self, load::SizedTexture, Color32, ColorImage, ImageData, TextureOptions},
    };

    use super::{chat_resources, player_resources, terrain_events, terrain_resources};

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
                let value = generator.sample_2d(
                    sample_position.try_into().unwrap(),
                    &generator.params.height_params,
                );
                let value = value * size.y as f64;
                let value = value as u8;
                data[(x + z * width) as usize] = value;
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

    pub fn regenerate_heightmap_system(
        mut events: EventReader<terrain_events::RegenerateHeightMapEvent>,
        generator: ResMut<terrain_resources::Generator>,
        mut noise_texture: ResMut<terrain_resources::NoiseTexture>,
        mut contexts: EguiContexts,
    ) {

        for _ in events.read() {
            let width = 1024;
            let height = 1024;
            let depth = 1024;

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

    pub fn render_visualizer_system(
        mut contexts: EguiContexts,
        noise_texture: ResMut<terrain_resources::NoiseTexture>,
        mut generator: ResMut<terrain_resources::Generator>,
        mut event_writer: EventWriter<terrain_events::RegenerateHeightMapEvent>,
    ) {
        match &noise_texture.texture {
            Some(texture_handle) => {
                egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
                    ui.label("world");

                    ui.add(egui::widgets::Slider::new(
                        &mut generator.params.height_params.octaves,
                        1..=8,
                    ));
                    ui.add(egui::widgets::Slider::new(
                        &mut generator.params.height_params.height,
                        0.0..=10.0,
                    ));
                    ui.add(egui::widgets::Slider::new(
                        &mut generator.params.height_params.lacuranity,
                        0.0..=4.0,
                    ));
                    ui.add(egui::widgets::Slider::new(
                        &mut generator.params.height_params.frequency,
                        0.0..=1.0,
                    ));
                    ui.add(egui::widgets::Slider::new(
                        &mut generator.params.height_params.amplitude,
                        0.0..=20.0,
                    ));
                    ui.add(egui::widgets::Slider::new(
                        &mut generator.params.height_params.persistence,
                        0.0..=1.0,
                    ));

                    if ui.button("Regenerate").clicked() {
                        event_writer.send(terrain_events::RegenerateHeightMapEvent);
                    };

                    ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                        texture_handle.id(),
                        texture_handle.size_vec2(),
                    )));
                });
            }
            None => {}
        }
    }
}
