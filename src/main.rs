use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        camera,
        render_resource::{AsBindGroup, ShaderRef},
    },
    window::WindowResolution,
};
use cgmath::{Matrix4, Rad, Vector3};
use iyes_perf_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(720.0, 720.0),
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
        .add_systems(Update, (animate, camera_controller))
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

    let chunk_size = 38;

    for x in 0..chunk_size {
        for z in 0..chunk_size {
            let mesh = meshes.add(Cuboid::default());
            let transform = Transform::from_xyz(
                (x - chunk_size / 2) as f32,
                0.0,
                (z - chunk_size / 2) as f32,
            );
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
        }
    }

    let camera_and_light_transform =
        Transform::from_xyz(0.0, 50.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y);

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

fn camera_controller(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    for (mut transform, camera) in query.iter_mut() {
        let mut translation = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            translation += Vec3::X;
        }
        if keys.pressed(KeyCode::KeyA) {
            translation -= Vec3::Z;
        }
        if keys.pressed(KeyCode::KeyS) {
            translation -= Vec3::X;
        }
        if keys.pressed(KeyCode::KeyD) {
            translation += Vec3::Z;
        }

        if keys.pressed(KeyCode::Space) {
            translation += Vec3::Y;
        }

        if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            translation -= Vec3::Y;
        }

        if translation != Vec3::ZERO {
            let translation = translation.normalize() * 4.0 * time.delta_seconds();
            transform.translation += translation;
        }
    }
}
