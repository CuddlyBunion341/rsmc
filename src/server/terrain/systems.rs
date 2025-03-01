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

#[cfg(feature = "generator_visualizer")]
pub use visualizer::*;

#[cfg(feature = "generator_visualizer")]
mod visualizer {
    use bevy::{
        log::{info, warn},
        math::{Vec2, Vec3},
        prelude::{EventReader, EventWriter, ResMut},
    };
    use bevy_inspector_egui::{
        bevy_egui::EguiContexts,
        egui::{self, Color32, ColorImage, ImageData, TextureOptions},
    };
    use bevy_renet::netcode::generate_random_bytes;
    use egui_plot::{Line, PlotPoint, PlotPoints};
    use rayon::iter::IntoParallelIterator;

    use rayon::iter::ParallelIterator;
    use renet::{DefaultChannel, RenetServer};
    use rsmc::{Chunk, ChunkManager, NetworkingMessage, CHUNK_SIZE};

    use super::{
        terrain_events,
        terrain_resources::{self, NoiseFunctionParams, TextureType},
    };

    fn map_range(value: f64, min: f64, max: f64, new_min: f64, new_max: f64) -> f64 {
        ((value - min) / (max - min)) * (new_max - new_min) + new_min
    }

    fn generate_terrain_heightmap(
        generator: &terrain_resources::Generator,
        texture_type: &TextureType,
        size: Vec3,
        draw_chunk_border: bool,
    ) -> ImageData {
        let mut data = vec![0; (size.x * size.z) as usize];

        let width = size.x as usize;
        let height = size.z as usize;

        for x in 0..width {
            for z in 0..height {
                let index = x + z * width;

                if draw_chunk_border && (x % CHUNK_SIZE == 0 || z % CHUNK_SIZE == 0) {
                    data[index] = 255;
                    continue;
                }

                match texture_type {
                    TextureType::Height => {
                        let sample_position = Vec2::new(x as f32, z as f32);
                        let value = generator.normalized_spline_terrain_sample(sample_position);
                        let value = (value * size.y as f64) / 2.0 + 0.5;

                        data[index] = value as u8;
                    }
                    TextureType::HeightAdjust => {
                        let sample_position = Vec2::new(x as f32, z as f32);
                        let value = generator
                            .sample_2d(sample_position, &generator.params.height_adjust.noise);
                        let value = map_range(value, -1.0, 1.0, 0.0, 255.0);

                        data[index] = value as u8;
                    }
                    TextureType::Density => {
                        let pos = Vec3::new(x as f32, z as f32, 0.0);
                        let value = generator.sample_3d(pos, &generator.params.density.noise);
                        let value = map_range(value, -1.0, 1.0, 0.0, 255.0);

                        data[index] = value as u8;
                    }
                    TextureType::Cave => {
                        let pos = Vec3::new(x as f32, z as f32, 0.0);
                        let mut value = generator.sample_3d(pos, &generator.params.cave.noise);

                        let base = generator.params.cave.base_value;
                        let upper_bound = base + generator.params.cave.threshold;
                        let lower_bound = base - generator.params.cave.threshold;

                        if lower_bound <= value && value >= upper_bound {
                            value = -1.0;
                        }

                        let value = map_range(value, -1.0, 1.0, 0.0, 255.0);

                        data[index] = value as u8;
                    }
                };
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
        mut noise_texture_list: ResMut<terrain_resources::NoiseTextureList>,
        mut contexts: EguiContexts,
    ) {
        for event in events.read() {
            let texture_type = event.0.clone();

            info!("Regenerating noise preview for {:?}", texture_type);

            let width = 512;
            let height = 512;
            let depth = 512;

            let image_data = generate_terrain_heightmap(
                &generator,
                &texture_type,
                Vec3::new(width as f32, height as f32, depth as f32),
                true,
            );

            let entry = noise_texture_list
                .noise_textures
                .get_mut(&texture_type)
                .expect("Noise texture not loaded, please initialize the resource properly.");

            entry.texture = Some(contexts.ctx_mut().load_texture(
                    "terrain-texture",
                    image_data,
                    TextureOptions::default(),
            ));
            entry.size = Vec2::new(width as f32, height as f32);
        }
    }

    #[rustfmt::skip]
    pub fn prepare_visualizer_texture_system(
        mut event_writer: EventWriter<terrain_events::RegenerateHeightMapEvent>,
    ) {
        event_writer.send(terrain_events::RegenerateHeightMapEvent(TextureType::Height));
        event_writer.send(terrain_events::RegenerateHeightMapEvent(TextureType::HeightAdjust));
        event_writer.send(terrain_events::RegenerateHeightMapEvent(TextureType::Density));
        event_writer.send(terrain_events::RegenerateHeightMapEvent(TextureType::Cave));
    }

    macro_rules! add_slider {
        ($ui:expr, $changed:expr, $value:expr, $range:expr, $text:expr) => {{
            $changed = $changed
                || $ui
                .add(egui::widgets::Slider::new($value, $range).text($text))
                .changed();
            }};
    }

    macro_rules! add_noise_sliders {
        ($ui:expr, $changed:expr, $params:expr) => {
            add_slider!($ui, $changed, &mut $params.octaves, 1..=8, "octaves");
            add_slider!($ui, $changed, &mut $params.lacuranity, 0.001..=4.0, "lacuranity");
            add_slider!($ui, $changed, &mut $params.frequency, 10.0..=800.0, "frequency");
            add_slider!($ui, $changed, &mut $params.persistence, 0.001..=1.0, "persistence");
        };
    }

    macro_rules! add_sliders_for_noise_params {
        ($ui:expr, $changed:expr, $params:expr) => {
            $params.frequency = 1.0 / $params.frequency;
            add_noise_sliders!($ui, *$changed, $params);
            $params.frequency = 1.0 / $params.frequency;
        };
    }

    // #[rustfmt::skip]
    // fn add_sliders_for_noise_params(
    //     ui: &mut egui::Ui,
    //     changed: &mut bool,
    //     params: &mut NoiseFunctionParams,
    // ) {
    //     params.frequency = 1.0 / params.frequency;
    //     add_noise_sliders!(ui, *changed, params);
    //     params.frequency = 1.0 / params.frequency;
    // }

    #[rustfmt::skip]
    pub fn render_visualizer_system(
        mut contexts: EguiContexts,
        noise_texture_list: ResMut<terrain_resources::NoiseTextureList>,
        mut generator: ResMut<terrain_resources::Generator>,
        mut event_writer: EventWriter<terrain_events::RegenerateHeightMapEvent>,
        mut world_regenerate_event_writer: EventWriter<terrain_events::WorldRegenerateEvent>,
    ) {
        egui::Window::new("Splines").show(contexts.ctx_mut(), |ui| {
            if ui.button("Organize windows").clicked() {
                ui.ctx().memory_mut(|mem| mem.reset_areas());
            }

            let mut changed = false;

            let length = generator.params.height.splines.len();

            for index in 0..length {
                if index != 0 && index != length - 1 {
                    // Ensure range from 0 to 1 by locking the first and last splines
                    add_slider!(ui, changed, &mut generator.params.height.splines[index].x, -1.0..=1.0, format!("x{}", index));
                }
                add_slider!(ui, changed, &mut generator.params.height.splines[index].y, -40.0..=80.0, format!("y{}", index));
            }

            if changed {
                event_writer.send(terrain_events::RegenerateHeightMapEvent(TextureType::Height));
            }

            if ui.button("Regenerate world").clicked() {
                world_regenerate_event_writer.send(terrain_events::WorldRegenerateEvent);
            }

            egui_plot::Plot::new("splines")
                .show(ui, |plot_ui| {
                    let plot_points: Vec<PlotPoint> = generator.params.height.splines.iter().map(|spline| PlotPoint {x: spline.x as f64, y: spline.y as f64}).collect();
                    let line_chart = Line::new(PlotPoints::Owned(plot_points));
                    plot_ui.line(line_chart);
                });
        });

        let noise_textures = &noise_texture_list.noise_textures;

        for (texture_type, noise_texture) in noise_textures {
            let texture_handle = noise_texture.texture.as_ref();

            match texture_handle {
                None => {
                    warn!("Noise texture handle could not be borrowed")
                },
                Some(texture_handle) => {
                    let window_name = match texture_type {
                        TextureType::Height => "Base Height",
                        TextureType::HeightAdjust => "Height adjustment",
                        TextureType::Density => "Density",
                        TextureType::Cave => "Cave",
                    };


                    egui::Window::new(window_name).show(contexts.ctx_mut(), |ui| {
                        ui.label(window_name);

                        let mut changed = false;

                        let params: &mut NoiseFunctionParams = match texture_type {
                            TextureType::Height => &mut generator.params.height.noise,
                            TextureType::HeightAdjust => &mut generator.params.height_adjust.noise,
                            TextureType::Density => {
                                generator.params.density.squash_factor = 1.0 / generator.params.density.squash_factor;
                                add_slider!(ui, changed, &mut generator.params.density.squash_factor, 10.0..=500.0, "squash factor");
                                generator.params.density.squash_factor = 1.0 / generator.params.density.squash_factor;
                                &mut generator.params.density.noise 
                            }
                            TextureType::Cave => {
                                add_slider!(ui, changed, &mut generator.params.cave.base_value, -1.0..=1.0, "base value");
                                add_slider!(ui, changed, &mut generator.params.cave.threshold, -1.0..=1.0, "treshold");
                                &mut generator.params.cave.noise
                            },
                        };

                        add_sliders_for_noise_params!(ui, &mut changed, params);

                        if changed {
                            event_writer.send(terrain_events::RegenerateHeightMapEvent(texture_type.clone()));
                        };

                        ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                                    texture_handle.id(),
                                    texture_handle.size_vec2(),
                        )));
                    });
                }
            }

        };
    }
}
