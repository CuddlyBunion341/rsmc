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

use bevy::color::palettes::css::WHITE;
use bevy_flair::FlairPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use scene::setup_scene;
use wireframe::WireframeConfig;

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

    App::new()
        .add_plugins((
            default_plugins,
            FlairPlugin,
            #[cfg(feature = "wireframe")]
            WireframePlugin,
            WorldInspectorPlugin::default(),
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
        ))
        .insert_state(GameState::Playing)
        .insert_resource(WireframeConfig {
            #[cfg(not(feature = "wireframe"))]
            global: false,
            #[cfg(feature = "wireframe")]
            global: true,
            default_color: WHITE.into(),
        })
        .add_systems(Startup, setup_scene)
        .run();
}
