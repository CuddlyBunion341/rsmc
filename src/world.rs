use crate::chunk::CHUNK_SIZE;
use crate::chunk_manager::ChunkManager;
use crate::{chunk::Chunk, generator::Generator, MyCube};
use crate::{mesher::*, MyChunk};
use bevy::asset::AssetServer;
use bevy::ecs::system::Res;
use bevy::math::Vec3;
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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let generator = Generator::new(0);

    let render_distance = 16;

    let texture_handle = asset_server.load("textures/texture_atlas.png");

    let mut chunks = ChunkManager::instantiate_chunks(Vec3::new(0.0, 0.0, 0.0), render_distance);

    for chunk in &mut chunks {
        generator.generate_chunk(chunk);
        let mesh = create_chunk_mesh(&chunk);

        let transform = Transform::from_xyz(
            chunk.position.x * CHUNK_SIZE as f32,
            0.0,
            chunk.position.z * CHUNK_SIZE as f32,
        );

        let material = materials.add(StandardMaterial {
            perceptual_roughness: 0.5,
            reflectance: 0.0,
            unlit: false,
            specular_transmission: 0.0,
            base_color_texture: Some(texture_handle.clone()),
            ..default()
        });

        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(mesh),
                transform,
                material,
                ..default()
            },
            MyCube,
            MyChunk {
                position: [
                    chunk.position.x as i32,
                    chunk.position.y as i32,
                    chunk.position.z as i32,
                ],
            },
        ));
    }

    chunk_manager.insert_chunks(chunks);
}
