use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::primitives::Cuboid,
    pbr::{MaterialMeshBundle, StandardMaterial},
    render::mesh::Mesh,
    transform::components::Transform,
};
use noise::{NoiseFn, Perlin};

use crate::MyCube;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let perlin = Perlin::new(1);
    let chunk_size = 32;

    for x in 0..chunk_size {
        for z in 0..chunk_size {
            let mesh = meshes.add(Cuboid::default());
            let height = perlin.get([x as f64 / 10.0, z as f64 / 10.0]) * 3.0;
            let transform = Transform::from_xyz(
                (x - chunk_size / 2) as f32,
                height.floor() as f32,
                (z - chunk_size / 2) as f32,
            );
            let material = materials.add(StandardMaterial {
                ..Default::default()
            });

            commands.spawn((
                MaterialMeshBundle {
                    mesh,
                    transform,
                    material,
                    ..Default::default()
                },
                MyCube,
            ));
        }
    }
}
