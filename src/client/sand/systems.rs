use crate::prelude::*;

pub fn spawn_falling_blocks_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Cuboid::new(1.0, 1.0, 1.0);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgba(1.0, 1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(0.0, 20.0, 0.0),
            ..default()
        })
    .insert(sand_components::FallingBlock());
}
//
// pub fn update_falling_blocks_system(
//     mut query: Query<&Transform, sand_components::FallingBlock>
// ) {
// }
