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
use bevy::color::palettes::css::WHITE;
#[cfg(feature = "wireframe")]
use wireframe::WireframeConfig;
#[cfg(feature = "wireframe")]
use wireframe::WireframePlugin;

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
        WireframePlugin,
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
    app.insert_resource(WireframeConfig {
        #[cfg(not(feature = "wireframe"))]
        global: false,
        #[cfg(feature = "wireframe")]
        global: true,
        default_color: WHITE.into(),
    });

    app.add_systems(Startup, setup_scene).run();
}
