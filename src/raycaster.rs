use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Query, ResMut, Resource},
    },
    gizmos::gizmos::Gizmos,
    math::{primitives::Cuboid, Ray3d, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    prelude::{default, Deref, DerefMut},
    render::{color::Color, mesh::Mesh},
    transform::components::Transform,
};
use bevy_mod_raycast::immediate::{Raycast, RaycastSettings};
use smooth_bevy_cameras::controllers::fps::FpsCameraController;

#[derive(Resource, Deref, DerefMut)]
pub struct SelectedPosition(pub Option<Vec3>);

#[derive(Resource)]
pub struct BlockSelection {
    pub position: Option<Vec3>,
    pub normal: Option<Vec3>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SelectedNormal(pub Option<Vec3>);

const RAY_DIST: Vec3 = Vec3::new(0.0, 0.0, -20.0);

// query camera position and direction
pub fn raycast(
    mut raycast: Raycast,
    mut gizmos: Gizmos,
    query: Query<&Transform, With<FpsCameraController>>,
    mut highlight_query: Query<(&mut Transform, &HighlightCube), Without<FpsCameraController>>,
    mut block_selection: ResMut<BlockSelection>,
) {
    let camera_transform = query.single();
    let filter = |entity| !highlight_query.contains(entity);

    let pos = camera_transform.translation;
    let dir = camera_transform.rotation.mul_vec3(Vec3::Z).normalize();
    let dir = dir * RAY_DIST.z;

    let intersections = raycast.debug_cast_ray(
        Ray3d::new(pos, dir),
        &RaycastSettings {
            filter: &filter,
            ..default()
        },
        &mut gizmos,
    );

    let (mut highlight_transform, _) = highlight_query.single_mut();
    let hover_position = intersections
        .first()
        .map(|(_, intersection)| (intersection.position() - intersection.normal() * 0.5).floor());

    block_selection.position = hover_position.clone();
    block_selection.normal = intersections
        .first()
        .map(|(_, intersection)| intersection.normal());

    if hover_position.is_none() {
        highlight_transform.translation = Vec3::new(-100.0, -100.0, -100.0);
        return;
    }

    highlight_transform.translation = hover_position.unwrap() + 0.5;
}

#[derive(Component)]
pub struct HighlightCube;

pub fn add_highlight_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Cuboid::new(1.01, 1.01, 1.01);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.5)),
            transform: Transform::from_xyz(0.0, 0.0, -7.0),
            ..default()
        })
        .insert(HighlightCube);
}
