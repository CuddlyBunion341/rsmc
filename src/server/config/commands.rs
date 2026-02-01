use crate::prelude::*;
use std::path::Path;

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    #[command(about = "Initialize config file with default settings")]
    Init,
    #[command(about = "Show currently applied configuration")]
    Show,
    #[command(about = "Show defaults")]
    Defaults,
}

pub fn perform_command(commands: &ConfigCommands) {
    match commands {
        ConfigCommands::Init => {
            if Path::new(&CONFIG_PATH).is_file() {
                eprintln!("Config file is already initialized at '{CONFIG_PATH}'.");
                eprintln!("If you want to reinitialize it with defaults, remove it:");
                eprintln!("rm '{CONFIG_PATH}'");
            } else {
                let config = Config::default();
                let config_str =
                    toml::to_string(&config).expect("Default config should be serializable");

                if let Err(err) = std::fs::write(CONFIG_PATH, config_str) {
                    eprintln!("Error writing to file: {err}");
                } else {
                    println!("Initialized config file '{CONFIG_PATH}'");
                }
            }
        }
        ConfigCommands::Show => {
            println!(
                "{}",
                toml::to_string(&*CONFIG).expect("Loaded config should always be serializable")
            );
        }
        ConfigCommands::Defaults => {
            println!(
                "{}",
                toml::to_string(&Config::default())
                    .expect("Loaded config should always be serializable")
            );
        }
    }
}
