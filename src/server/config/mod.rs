use std::sync::LazyLock;

pub mod commands;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    // TODO: read from file
    Config::default()
});

use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    pub world: WorldConfig,
}

#[derive(Deserialize)]
pub struct WorldConfig {
    pub backups_dir: String,
    pub worlds_dir: String,
    pub world_extension: String,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            backups_dir: String::from("backups/"),
            worlds_dir: String::from("worlds/"),
            world_extension: String::from(".rsmcw"),
        }
    }
}
