use std::sync::LazyLock;

pub mod commands;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    #[cfg(test)]
    {
        Config::default()
    }

    #[cfg(not(test))]
    {
        use std::path::PathBuf;
        const CONFIG_PATH: &str = "server.toml";

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
