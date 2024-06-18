use bevy::asset::{AssetServer, Handle};
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::{Query, Res};
use bevy::math::Vec3;
use bevy::render::texture::Image;
use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    pbr::{MaterialMeshBundle, StandardMaterial},
    prelude::default,
    render::mesh::Mesh,
    transform::components::Transform,
};

use super::chunk::{Chunk, CHUNK_SIZE};
use super::generator::Generator;
use super::mesher::create_chunk_mesh;
use super::{ChunkManager, ChunkMesh, ChunkMeshUpdateEvent};

pub fn setup_world_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let generator = Generator::new(0);

    let render_distance = 16;

    let mut chunks = ChunkManager::instantiate_chunks(Vec3::new(0.0, 0.0, 0.0), render_distance);

    for chunk in &mut chunks {
        generator.generate_chunk(chunk);
        let mesh = create_chunk_mesh(chunk);
        add_chunk_objects(
            &mut commands,
            &asset_server,
            &mut meshes,
            &mut materials,
            chunk,
        );
    }

    chunk_manager.insert_chunks(chunks);
}

pub fn handle_chunk_mesh_update_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_mesh_update_events: EventReader<ChunkMeshUpdateEvent>,
    mut mesh_query: Query<(Entity, &ChunkMesh)>,
) {
    for event in chunk_mesh_update_events.read() {
        let chunk_option = chunk_manager.get_chunk(event.position);
        match chunk_option {
            Some(chunk) => {
                for (entity, chunk_mesh) in mesh_query.iter_mut() {
                    if Chunk::key_eq_pos(chunk_mesh.key, chunk.position) {
                        commands.entity(entity).despawn();
                    }
                }
                add_chunk_objects(
                    &mut commands,
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    chunk,
                );
            }
            None => {
                println!("No chunk found");
            }
        }
    }
}


fn add_chunk_objects(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk: &Chunk,
) {
    let texture_handle: Handle<Image> = asset_server.load("textures/texture_atlas.png");
    let mesh_option = create_chunk_mesh(chunk);

    if mesh_option.is_none() {
        return;
    }

    let mesh = mesh_option.unwrap();

    let transform = Transform::from_xyz(
        chunk.position.x * CHUNK_SIZE as f32,
        chunk.position.y * CHUNK_SIZE as f32,
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
        ChunkMesh {
            key: [
                chunk.position.x as i32,
                chunk.position.y as i32,
                chunk.position.z as i32,
            ],
        },
    ));
}
