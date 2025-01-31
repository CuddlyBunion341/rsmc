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
        asset::RenderAssetUsages, image::Image, log::info, math::{Vec2, Vec3}, prelude::Res, render::render_resource::{Extent3d, TextureDimension, TextureFormat}
    };
    use bevy_inspector_egui::{
        bevy_egui::EguiContexts,
        egui::{self, load::SizedTexture, Color32, ColorImage, ImageData, TextureOptions},
    };

    use super::terrain_resources;

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
                let sample_position = Vec2::new((origin.x + x as f32) / 20.0, (origin.z + z as f32) / 20.0);
                let value = generator.sample_2d(
                    sample_position.try_into().unwrap(),
                    &generator.params.height_params,
                );
                let value = value * size.y as f64;
                let value = value as u8;
                data[(x + z * width) as usize] = value;
            }
        }

        let color_data: Vec<Color32> = data.iter().map(|&value| Color32::from_gray(value)).collect();

        let color_image: ColorImage = ColorImage {
            size: [width, height],
            pixels: color_data,
        };

        ImageData::Color(color_image.into())
    }

    pub fn render_visualizer_system(
        mut contexts: EguiContexts,
        generator: Res<terrain_resources::Generator>,
    ) {
        let image_data =
            generate_terrain_heightmap(&generator, Vec3::ZERO, Vec3::new(128.0, 128.0, 128.0));

        let texture_handle = contexts.ctx_mut().load_texture("Foo", image_data, TextureOptions::default());

        egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
            ui.label("world");
            ui.image(&texture_handle);
        });
    }
}
