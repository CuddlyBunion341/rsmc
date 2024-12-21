use sand_components::FallingBlock;

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
    .insert(sand_components::FallingBlock {
        lifetime: 5000,
    });
}

pub fn remove_old_entities_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut sand_components::FallingBlock)>
) {
    for (entity, falling_block) in query.iter() {
        if falling_block.lifetime <= 0 {
            // TODO: implement kill entity
        }
    }
}

pub fn tick_falling_blocks_system(
    mut query: Query<(&mut sand_components::FallingBlock, &mut Transform)>
) {

    for (mut falling_block, mut transform) in query.iter_mut() {
        falling_block.lifetime -= 1;
        transform.translation.y -= 0.01;
    }
}
