use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use iyes_perf_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(PerfUiCompleteBundle::default());

    commands.spawn((MaterialMeshBundle {
        mesh: meshes.add(Cuboid::default()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(CustomMaterial {}),
        ..default()
    },));

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(1.8, 1.8, 1.8).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands.spawn(Camera3dBundle {
        transform: camera_and_light_transform,
        ..default()
    });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animate_shader.wgsl".into()
    }
}
