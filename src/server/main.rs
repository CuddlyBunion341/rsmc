pub mod chat;
pub mod config;
pub mod networking;
pub mod player;
pub mod prelude;
pub mod terrain;

use bevy::app::TerminalCtrlCHandlerPlugin;

#[cfg(feature = "egui_layer")]
use bevy::DefaultPlugins;
#[cfg(feature = "egui_layer")]
pub mod gui;

#[cfg(not(feature = "egui_layer"))]
use bevy::log::LogPlugin;

use crate::prelude::*;

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
                Err(error) => return eprintln!("Error: {}", error),
            };
        }
        MainCommands::Config(config_commands) => {
            config_commands::perform_command(&config_commands);
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
