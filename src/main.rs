use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use cgmath::{Matrix4, Rad, Vector3};
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
        .add_systems(Update, animate)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct MyCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(PerfUiCompleteBundle::default());

    let mesh = meshes.add(Cuboid::default());
    let transform = Transform::from_xyz(0.0, 0.0, 0.0);
    let material = materials.add(CustomMaterial {});

    commands.spawn((
        MaterialMeshBundle {
            mesh,
            transform,
            material,
            ..default()
        },
        MyCube,
    ));

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

fn animate(time: Res<Time>, mut query: Query<(&mut Transform, &MyCube)>) {
    let rotation_change = time.delta_seconds() * 5.0;

    for (mut transform, cube) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(Rad(rotation_change).0));
    }
}
