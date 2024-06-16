use bevy::prelude::*;
use bevy::window::*;
use iyes_perf_ui::prelude::*;
use bevy::diagnostic::*;

mod terrain;
mod player;
mod collider;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1920.0, 1080.0)
                            .with_scale_factor_override(2.0),
                        present_mode: bevy::window::PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(EntityCountDiagnosticsPlugin)
        .add_plugins(SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_plugins(terrain::TerrainPlugin)
        .add_plugins(collider::ColliderPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}
