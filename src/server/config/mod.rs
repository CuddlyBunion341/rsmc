use std::sync::LazyLock;

pub mod commands;

pub const CONFIG_PATH: &str = "server.toml";

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    #[cfg(test)]
    {
        Config::default()
    }

    #[cfg(not(test))]
    {
        use std::path::PathBuf;

        match std::fs::read_to_string(PathBuf::from(CONFIG_PATH)) {
            Ok(string) => match toml::from_str(&string) {
                Ok(config) => config,
                Err(_) => panic!("Could not parse config file at '{CONFIG_PATH}'"),
            },
            Err(_) => {
                eprintln!(
                    "Could not read config file at '{CONFIG_PATH}', proceeding with defaults"
                );
                Config::default()
            }
        }
    }
});

use serde::Deserialize;
use serde::Serialize;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub world: terrain_config::WorldConfig,
    pub generator: terrain_config::TerrainGeneratorParams,
}
