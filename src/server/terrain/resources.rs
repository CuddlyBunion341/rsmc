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

impl Default for Generator {
    fn default() -> Self {
        Self::new(0)
    }
}

pub struct TerrainGeneratorParams {
    pub height_params: NoiseFunctionParams,
    pub density_params: NoiseFunctionParams,
}

impl Default for TerrainGeneratorParams {
    fn default() -> Self {
        Self {
            height_params: NoiseFunctionParams {
                octaves: 4,
                height: 0.0,
                lacuranity: 2.0,
                frequency: 1.0 / 60.0,
                amplitude: 10.0,
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
