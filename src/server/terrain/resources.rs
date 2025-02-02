use crate::prelude::*;

use bevy_inspector_egui::egui::TextureHandle;
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

#[derive(Debug, Copy, Clone)]
pub struct NoiseFunctionParams {
    pub octaves: i32,
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
    pub splines: Vec<Vec2>,
    pub height_params: NoiseFunctionParams,
    pub density_params: NoiseFunctionParams,
}

impl Default for TerrainGeneratorParams {
    fn default() -> Self {
        Self {
            splines: vec![
                Vec2::new(-1.0, 0.0),
                Vec2::new(1.0, 50.0),
            ],
            height_params: NoiseFunctionParams {
                octaves: 4,
                height: 0.0,
                lacuranity: 2.0,
                frequency: 1.0 / 120.0,
                amplitude: 30.0,
                persistence: 0.5,
            },
            density_params: NoiseFunctionParams {
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

// visualizer

#[derive(Resource)]
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
