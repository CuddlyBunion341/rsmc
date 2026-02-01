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
                Err(err) => {
                    use std::process;

                    eprintln!("Could not parse config file at '{CONFIG_PATH}'");
                    eprintln!("Err: {err}");
                    process::exit(1);
                }
            },
            Err(err) => {
                eprintln!(
                    "Could not read config file at '{CONFIG_PATH}', proceeding with defaults"
                );
                eprintln!("Err: {err}");
                Config::default()
            }
        }
    }
});

use serde::Deserialize;
use serde::Serialize;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub world: terrain_config::WorldConfig,
    pub generator: terrain_config::TerrainGeneratorParams,
}
