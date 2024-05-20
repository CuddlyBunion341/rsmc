use crate::chunk::CHUNK_SIZE;
use crate::mesher::*;
use crate::{chunk::Chunk, generator::Generator, MyCube};
use bevy::math::Vec3;
use bevy::pbr::Material;
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
    let generator = Generator::new(0);

    let render_distance = 16;

    for x in 0..render_distance {
        for z in 0..render_distance {
            let mut chunk = Chunk::new(Vec3 {
                x: x as f32,
                y: 0.0,
                z: z as f32,
            });

            generator.generate_chunk(&mut chunk);
            let mesh = create_chunk_mesh(chunk);

            let transform = Transform::from_xyz(
                (x - render_distance / 2) as f32 * CHUNK_SIZE as f32 * 2.0,
                0.0,
                (z - render_distance / 2) as f32 * CHUNK_SIZE as f32 * 2.0,
            );

            let material = materials.add(StandardMaterial { ..default() });

            commands.spawn((
                MaterialMeshBundle {
                    mesh: meshes.add(mesh),
                    transform,
                    material,
                    ..default()
                },
                MyCube,
            ));
        }
    }

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(create_cube_mesh_from_data(create_cube_geometry_data(
            0.0, 0.0, 0.0, 0b111111,
        ))),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(StandardMaterial { ..default() }),
        ..default()
    });
}
