use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy::window::*;
use iyes_perf_ui::prelude::*;
use scene::setup_scene;

mod collider;
mod networking;
mod player;
mod scene;
mod terrain;

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
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            PerfUiPlugin,
            networking::NetworkingPlugin,
            terrain::TerrainPlugin,
            collider::ColliderPlugin,
            player::PlayerPlugin,
        ))
        .add_systems(Startup, setup_scene)
        .run();
}
