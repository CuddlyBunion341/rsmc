pub mod chat;
pub mod config;
pub mod networking;
pub mod player;
pub mod prelude;
pub mod terrain;

use std::path::Path;

use bevy::app::TerminalCtrlCHandlerPlugin;
use clap::{Parser, Subcommand};

#[cfg(feature = "egui_layer")]
use bevy::DefaultPlugins;
#[cfg(feature = "egui_layer")]
pub mod gui;

#[cfg(not(feature = "egui_layer"))]
use bevy::log::LogPlugin;

use crate::{
    config::{Config, CONFIG, CONFIG_PATH},
    prelude::*,
};

#[derive(Debug, Parser)]
#[command(version)]
#[command(long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: MainCommands,
}

#[derive(Debug, Subcommand)]
enum MainCommands {
    #[command(flatten)]
    World(terrain_commands::WorldCommands),
    #[command(subcommand, about = "Actions regarding server configuration")]
    Config(config_commands::ConfigCommands),
}

fn main() {
    let mut app = App::new();
    app.add_plugins(TerminalCtrlCHandlerPlugin);

    #[cfg(not(feature = "egui_layer"))]
    {
        app.add_plugins(MinimalPlugins);
        app.add_plugins(LogPlugin::default());
    }

    #[cfg(feature = "egui_layer")]
    {
        use bevy_egui::EguiPlugin;
        app.add_plugins(DefaultPlugins);
        app.add_plugins(EguiPlugin::default());
        app.add_systems(Startup, gui::setup_camera_system);
    }

    let args = Cli::parse();
    match args.commands {
        MainCommands::World(world_commands) => {
            match terrain::TerrainPlugin::from_command(world_commands) {
                Ok(terrain_plugin) => app.add_plugins(terrain_plugin),
                Err(error) => {
                    eprintln!("Error: {}", error);
                    return;
                }
            };
        }
        MainCommands::Config(config_commands) => {
            match config_commands {
                config_commands::ConfigCommands::Init => {
                    if Path::new(&CONFIG_PATH).is_file() {
                        eprintln!("Config file is already initialized at '{CONFIG_PATH}'.");
                        eprintln!("If you want to reinitialize it with defaults, remove it:");
                        eprintln!("rm '{CONFIG_PATH}'");
                    } else {
                        let config = Config::default();
                        let config_str = toml::to_string(&config)
                            .expect("Default config should be serializable");

                        if let Err(err) = std::fs::write(CONFIG_PATH, config_str) {
                            eprintln!("Error writing to file: {err}");
                        } else {
                            println!("Initialized config file '{CONFIG_PATH}'");
                        }
                    }
                }
                config_commands::ConfigCommands::Show => {
                    println!(
                        "{}",
                        toml::to_string(&*CONFIG)
                            .expect("Loaded config should always be serializable")
                    );
                }
                config_commands::ConfigCommands::Defaults => {
                    println!(
                        "{}",
                        toml::to_string(&Config::default())
                            .expect("Loaded config should always be serializable")
                    );
                }
            }
            return;
        }
    }

    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(networking::NetworkingPlugin);

    #[cfg(feature = "chat")]
    app.add_plugins(chat::ChatPlugin);

    println!("Server is starting!");
    app.run();
}
