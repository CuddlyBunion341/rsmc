use crate::prelude::*;

pub mod prelude;

mod chat;
mod collider;
mod gui;
mod networking;
mod player;
mod remote_player;
mod scene;
mod states;
mod terrain;

use bevy_flair::FlairPlugin;
use scene::setup_scene;

#[cfg(feature = "wireframe")]
mod wireframe_config {
    use bevy::color::palettes::css::WHITE;
    use crate::wireframe::{WireframeConfig, WireframePlugin};

    pub fn wireframe_plugin() -> WireframePlugin {
        WireframePlugin
    }

    pub fn wireframe_config() -> WireframeConfig {
        WireframeConfig {
            global: true,
            default_color: WHITE.into(),
        }
    }
}

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1920.0, 1080.0).with_scale_factor_override(2.0),
            present_mode: bevy::window::PresentMode::Immediate,
            ..default()
        }),
        ..default()
    };

    let default_plugins = DefaultPlugins
        .set(window_plugin)
        .set(ImagePlugin::default_nearest());

    let mut app = App::new();
    app.add_plugins((
        default_plugins,
        FlairPlugin,
        #[cfg(feature = "wireframe")]
        wireframe_config::wireframe_plugin(),
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
        PerfUiPlugin,
        gui::GuiPlugin,
        networking::NetworkingPlugin,
        terrain::TerrainPlugin,
        collider::ColliderPlugin,
        player::PlayerPlugin,
        remote_player::RemotePlayerPlugin,
        chat::ChatPlugin,
    ));
    app.insert_state(GameState::Playing);

    #[cfg(feature = "wireframe")]
    app.insert_resource(wireframe_config::wireframe_config());

    app.add_systems(Startup, setup_scene).run();
}
