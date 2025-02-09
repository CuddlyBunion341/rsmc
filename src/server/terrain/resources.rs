use crate::prelude::*;

use terrain_events::BlockUpdateEvent;

#[derive(Resource)]
pub struct PastBlockUpdates {
    pub updates: Vec<BlockUpdateEvent>,
}

impl Default for PastBlockUpdates {
    fn default() -> Self {
        Self::new()
    }
}

impl PastBlockUpdates {
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
        }
    }
}

#[derive(Resource)]
pub struct Generator {
    pub seed: u32,
    pub perlin: Perlin, // TODO: reduce visibility of attributes on this struct decl
    pub params: TerrainGeneratorParams,
}

pub struct HeightParams {
    pub noise: NoiseFunctionParams,
    pub splines: Vec<Vec2>,
}

pub struct DensityParams {
    pub noise: NoiseFunctionParams,
    pub squash_factor: f64
}

#[derive(Debug)]
pub struct NoiseFunctionParams {
    pub octaves: u32,
    pub height: f64,
    pub lacuranity: f64,
    pub frequency: f64,
    pub amplitude: f64,
    pub persistence: f64,
}

impl Default for Generator {
    fn default() -> Self {
        Self::new(0)
    }
}

pub struct TerrainGeneratorParams {
    pub height_params: HeightParams,
    pub height_adjust_params: NoiseFunctionParams,
    pub density_params: DensityParams ,
}

impl Default for TerrainGeneratorParams {
    fn default() -> Self {
        Self {
            height_params: HeightParams {
                splines: vec![
                    Vec2::new(-1.0, -20.0),
                    Vec2::new(-0.5, 0.0),
                    Vec2::new(0.0, 20.0),
                    Vec2::new(0.5, 40.0),
                    Vec2::new(1.0, 60.0),
                ],
                noise: NoiseFunctionParams {
                    octaves: 4,
                    height: 0.0,
                    lacuranity: 2.0,
                    frequency: 1.0 / 120.0,
                    amplitude: 30.0,
                    persistence: 0.5,
                }
            },
            height_adjust_params: NoiseFunctionParams {
                octaves: 4,
                height: 0.0,
                lacuranity: 2.0,
                frequency: 1.0 / 120.0,
                amplitude: 30.0,
                persistence: 0.5,
            },
            density_params: DensityParams {
                    squash_factor: 1.0 / 200.0,
                    noise: NoiseFunctionParams {
                        octaves: 4,
                        height: 0.0,
                        lacuranity: 2.0,
                        frequency: 1.0 / 60.0,
                        amplitude: 10.0,
                        persistence: 0.5,
                    },
                }
        }
    }
}

#[cfg(feature = "generator_visualizer")]
pub use visualizer::*;

#[cfg(feature = "generator_visualizer")]
mod visualizer {
    use super::*;
    use bevy::utils::HashMap;
    use bevy_inspector_egui::egui::TextureHandle;

    #[derive(PartialEq, Hash, Eq, Clone, Debug)]
    pub enum TextureType {
        Height,
        HeightAdjust,
        Density,
    }

    #[derive(Resource)]
    pub struct NoiseTextureList {
        pub noise_textures: HashMap<TextureType, NoiseTexture>,
    }

    impl Default for NoiseTextureList {
        fn default() -> Self {
            let mut noise_textures = HashMap::new();

            noise_textures.insert(TextureType::Height, NoiseTexture::default());
            noise_textures.insert(TextureType::HeightAdjust, NoiseTexture::default());
            noise_textures.insert(TextureType::Density, NoiseTexture::default());

            NoiseTextureList { noise_textures }
        }
    }

    pub struct NoiseTexture {
        pub texture: Option<TextureHandle>,
        pub size: Vec2,
    }

    impl Default for NoiseTexture {
        fn default() -> Self {
            NoiseTexture {
                texture: None,
                size: Vec2::ZERO,
            }
        }
    }
}
