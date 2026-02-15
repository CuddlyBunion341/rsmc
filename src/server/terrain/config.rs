use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WorldConfig {
    pub backups_dir: String,
    pub worlds_dir: String,
    pub world_extension: String,
    pub world_save_interval_seconds: i64,
    pub world_backup_interval_seconds: i64,
    pub spawn_area_distance: IVec2,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            backups_dir: String::from("backups/"),
            worlds_dir: String::from("worlds/"),
            world_extension: String::from(".rsmcw"),
            world_save_interval_seconds: 30,
            world_backup_interval_seconds: 180,
            spawn_area_distance: IVec2::new(4, 4),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[derive(Default)]
pub struct TerrainGeneratorParams {
    pub height: HeightParams,
    pub height_adjust: HeightAdjustParams,
    pub density: DensityParams,
    pub cave: CaveParams,
    pub tree: TreeParams,
    pub grass: GrassParams,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct HeightParams {
    pub noise: NoiseFunctionParams,
    pub splines: Vec<Vec2>,
}

impl Default for HeightParams {
    fn default() -> Self {
        Self {
            splines: vec![
                Vec2::new(-1.0, 4.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(0.05, 20.0),
                Vec2::new(1.0, 35.0),
            ],
            noise: NoiseFunctionParams {
                octaves: 4,
                height: 0.0,
                lacuranity: 2.0,
                frequency: 1.0 / 300.0,
                amplitude: 30.0,
                persistence: 0.5,
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct HeightAdjustParams {
    pub noise: NoiseFunctionParams,
}

impl Default for HeightAdjustParams {
    fn default() -> Self {
        Self {
            noise: NoiseFunctionParams {
                octaves: 4,
                height: 0.0,
                lacuranity: 2.0,
                frequency: 1.0 / 120.0,
                amplitude: 30.0,
                persistence: 0.5,
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DensityParams {
    pub noise: NoiseFunctionParams,
    pub squash_factor: f64,
    pub height_offset: f64,
}

impl Default for DensityParams {
    fn default() -> Self {
        DensityParams {
            squash_factor: 1.0 / 100.0,
            height_offset: -20.0,
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

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CaveParams {
    pub noise: NoiseFunctionParams,
    pub base_value: f64,
    pub threshold: f64,
}

impl Default for CaveParams {
    fn default() -> Self {
        Self {
            noise: NoiseFunctionParams {
                octaves: 2,
                height: 0.0,
                lacuranity: 0.03,
                frequency: 1.0 / 20.0,
                amplitude: 30.0,
                persistence: 0.59,
            },
            base_value: 0.0,
            threshold: 0.25,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TreeParams {
    pub spawn_attempts_per_chunk: u32,
    pub min_stump_height: u32,
    pub max_stump_height: u32,
    pub min_bush_radius: u32,
    pub max_bush_radius: u32,
}

impl Default for TreeParams {
    fn default() -> Self {
        Self {
            spawn_attempts_per_chunk: 500,
            min_stump_height: 2,
            max_stump_height: 20,
            min_bush_radius: 3,
            max_bush_radius: 5,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrassParams {
    pub frequency: u32,
}

impl Default for GrassParams {
    fn default() -> Self {
        Self { frequency: 10 }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[derive(Default)]
pub struct NoiseFunctionParams {
    pub octaves: u32,
    pub height: f64,
    pub lacuranity: f64,
    pub frequency: f64,
    pub amplitude: f64,
    pub persistence: f64,
}
