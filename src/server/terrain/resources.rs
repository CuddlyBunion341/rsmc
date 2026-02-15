use crate::{prelude::*, terrain::config::TerrainGeneratorParams};

use std::collections::VecDeque;

use chrono::{DateTime, TimeDelta, Utc};
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use terrain_events::BlockUpdateEvent;

#[derive(Resource, Default)]
pub struct ClientChunkRequests {
    queues: HashMap<ClientId, VecDeque<ChunkPosition>>,
}

impl ClientChunkRequests {
    pub fn enqueue_bulk(
        &mut self,
        client_id: ClientId,
        chunk_positions: &mut VecDeque<ChunkPosition>,
    ) {
        self.queues
            .entry(client_id)
            .or_default()
            .append(chunk_positions);
    }

    pub fn remove(&mut self, client_id: &ClientId) {
        self.queues.remove(client_id);
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&ClientId, &mut VecDeque<ChunkPosition>) -> bool,
    {
        self.queues.retain(f)
    }
}

#[derive(Resource)]
pub struct AutoSaveName(pub String);

impl AutoSaveName {
    pub fn with_name(name: String) -> Self {
        Self(name)
    }

    pub fn with_random() -> Self {
        Self(Alphanumeric.sample_string(&mut rand::rng(), 16))
    }
}

#[derive(Resource)]
struct SaveTimer {
    pub last_autosave_timestamp: DateTime<Utc>,
    interval: chrono::TimeDelta,
}

impl SaveTimer {
    pub fn new(interval: TimeDelta) -> SaveTimer {
        SaveTimer {
            last_autosave_timestamp: Utc::now(),
            interval,
        }
    }
}

#[derive(Resource)]
pub struct WorldBackupTimer(SaveTimer);

impl WorldBackupTimer {
    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn is_ready(&self) -> bool {
        self.0.is_ready()
    }
}

impl Default for WorldBackupTimer {
    fn default() -> Self {
        Self(SaveTimer::new(TimeDelta::seconds(
            CONFIG.world.world_backup_interval_seconds,
        )))
    }
}

#[derive(Resource)]
pub struct WorldSaveTimer(SaveTimer);

impl WorldSaveTimer {
    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn is_ready(&self) -> bool {
        self.0.is_ready()
    }
}

impl Default for WorldSaveTimer {
    fn default() -> Self {
        Self(SaveTimer::new(TimeDelta::seconds(
            CONFIG.world.world_save_interval_seconds,
        )))
    }
}

impl SaveTimer {
    pub fn reset(&mut self) {
        self.last_autosave_timestamp = Utc::now();
    }

    pub fn is_ready(&self) -> bool {
        let timer_ready_timestamp = self
            .last_autosave_timestamp
            .checked_add_signed(self.interval)
            .expect("Time should never be out of range");
        timer_ready_timestamp < Utc::now()
    }
}

#[derive(Resource, Default)]
pub struct PastBlockUpdates {
    pub updates: Vec<BlockUpdateEvent>,
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Generator {
    pub noise: Noise,
    pub params: TerrainGeneratorParams,
}

impl Default for Generator {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Generator {
    pub fn with_seed(seed: u32) -> Self {
        Self::new(seed)
    }
}

#[derive(Clone)]
pub struct Noise {
    pub seed: u32,
    perlin: Perlin,
}

pub trait NoiseSample<T> {
    fn get(&self, position: T) -> f64;
}

impl Noise {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            perlin: Perlin::new(seed),
        }
    }
}

impl NoiseSample<DVec3> for Noise {
    fn get(&self, position: DVec3) -> f64 {
        self.perlin.get(position.to_array())
    }
}

impl NoiseSample<DVec2> for Noise {
    fn get(&self, position: DVec2) -> f64 {
        self.perlin.get(position.to_array())
    }
}

impl Serialize for Noise {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(self.seed)
    }
}

impl<'de> Deserialize<'de> for Noise {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seed = u32::deserialize(deserializer)?;
        Ok(Noise::new(seed))
    }
}

#[cfg(feature = "generator_visualizer")]
pub use visualizer::*;

#[cfg(feature = "generator_visualizer")]
mod visualizer {
    use super::*;
    use bevy_inspector_egui::egui::TextureHandle;

    #[derive(PartialEq, Hash, Eq, Clone, Debug)]
    pub enum TextureType {
        Height,
        HeightAdjust,
        Density,
        Cave,
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
            noise_textures.insert(TextureType::Cave, NoiseTexture::default());

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
