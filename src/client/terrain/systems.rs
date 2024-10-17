use crate::prelude::*;

pub fn setup_world_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<terrain_resources::ChunkManager>,
    texture_manager: ResMut<terrain_util::TextureManager>
) {
    let generator = terrain_util::generator::Generator::new(0);

    let render_distance = 16;

    let mut chunks = terrain_resources::ChunkManager::instantiate_chunks(
        Vec3::new(0.0, 0.0, 0.0),
        render_distance,
    );

    for chunk in &mut chunks {
        generator.generate_chunk(chunk);
        add_chunk_objects(
            &mut commands,
            &asset_server,
            &mut meshes,
            &mut materials,
            chunk,
            &texture_manager
        );
    }

    chunk_manager.insert_chunks(chunks);
}

pub fn handle_chunk_mesh_update_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<terrain_resources::ChunkManager>,
    mut chunk_mesh_update_events: EventReader<terrain_events::ChunkMeshUpdateEvent>,
    mut mesh_query: Query<(Entity, &terrain_components::ChunkMesh)>,
    texture_manager: ResMut<terrain_util::TextureManager>
) {
    for event in chunk_mesh_update_events.read() {
        let chunk_option = chunk_manager.get_chunk(event.position);
        match chunk_option {
            Some(chunk) => {
                for (entity, chunk_mesh) in mesh_query.iter_mut() {
                    if terrain_util::Chunk::key_eq_pos(chunk_mesh.key, chunk.position) {
                        commands.entity(entity).despawn();
                    }
                }
                add_chunk_objects(
                    &mut commands,
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    chunk,
                    &texture_manager
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
    chunk: &terrain_util::Chunk,
    texture_manager: &terrain_util::TextureManager,
) {
    let texture_handle: Handle<Image> = asset_server.load("textures/texture_atlas.png");
    let mesh_option = terrain_util::create_chunk_mesh(chunk, texture_manager);

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
        terrain_components::ChunkMesh {
            key: [
                chunk.position.x as i32,
                chunk.position.y as i32,
                chunk.position.z as i32,
            ],
        },
    ));
}
