use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::primitives::Cuboid,
    pbr::{MaterialMeshBundle, StandardMaterial},
    render::mesh::Mesh,
    transform::components::Transform,
};
use noise::{NoiseFn, Perlin};

use crate::mesher::*;
use crate::MyCube;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let perlin = Perlin::new(1);
    let chunk_size = 32;

    let mut geometry_data = GeometryData {
        position: Vec::new(),
        uv: Vec::new(),
        normal: Vec::new(),
        indices: Vec::new(),
    };

    for x in 0..chunk_size {
        for z in 0..chunk_size {
            let y = perlin.get([x as f64 / 10.0, z as f64 / 10.0]) * 3.0;
            let cube_data =
                create_cube_geometry_data(x as f32, y.floor() as f32, z as f32, 0b111111);

            geometry_data.position.extend(cube_data.position);
            geometry_data.uv.extend(cube_data.uv);
            geometry_data.normal.extend(cube_data.normal);
            geometry_data.indices.extend(
                cube_data
                    .indices
                    .iter()
                    .map(|i| i + geometry_data.position.len() as u32),
            );
        }
    }

    let mesh = create_cube_mesh_from_data(geometry_data);

    let transform = Transform::from_xyz(-(chunk_size / 2) as f32, 0.0, -(chunk_size / 2) as f32);

    let material = materials.add(StandardMaterial {
        ..Default::default()
    });

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
