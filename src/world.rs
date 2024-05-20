use crate::mesher::*;
use crate::{chunk::Chunk, generator::Generator, MyCube};
use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    pbr::{MaterialMeshBundle, StandardMaterial},
    prelude::default,
    render::mesh::Mesh,
    transform::components::Transform,
};

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut chunk = Chunk::new();
    let generator = Generator::new(0);

    generator.generate_chunk(&mut chunk);
    let mesh = create_chunk_mesh(chunk);

    let transform = Transform::from_xyz(0.0, 0.0, 0.0);

    let material = materials.add(StandardMaterial { ..default() });

    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(mesh),
            transform,
            material,
            ..Default::default()
        },
        MyCube,
    ));
}
