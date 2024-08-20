use crate::prelude::*;

const RAY_DIST: Vec3 = Vec3::new(0.0, 0.0, -20.0);

pub fn setup_highlight_cube_system(
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
        .insert(player_components::HighlightCube);
}

pub fn raycast_system(
    mut raycast: Raycast,
    mut debug_ui: ResMut<debug_ui_resources::DebugUi>,
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Camera3d>>,
    mut highlight_query: Query<
        (&mut Transform, &player_components::HighlightCube),
        Without<Camera3d>,
    >,
    mut block_selection: ResMut<player_resources::BlockSelection>,
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

    block_selection.position = hover_position;
    block_selection.normal = intersections
        .first()
        .map(|(_, intersection)| intersection.normal());

    if hover_position.is_none() {
        highlight_transform.translation = Vec3::new(-100.0, -100.0, -100.0);
        return;
    }

    highlight_transform.translation = hover_position.unwrap() + 0.5;

    debug_ui.selected_block = block_selection.position.unwrap();
}
