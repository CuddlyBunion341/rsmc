use crate::prelude::*;

pub fn setup_world_system(mut client: ResMut<RenetClient>) {
    let render_distance = 16;

    info!("Sending chunk requests for chunks");

    let chunks = terrain_resources::ChunkManager::instantiate_chunks(
        Vec3::new(0.0, 0.0, 0.0),
        render_distance,
    );

    let positions = chunks.into_iter().map(|chunk| chunk.position).collect();
    debug!("Sending chunk batch request for {:?}", positions);
    let message = bincode::serialize(&NetworkingMessage::ChunkBatchRequest(positions));
    client.send_message(DefaultChannel::ReliableUnordered, message.unwrap());
}

#[allow(clippy::too_many_arguments)]
pub fn handle_chunk_mesh_update_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<terrain_resources::ChunkManager>,
    mut chunk_mesh_update_events: EventReader<terrain_events::ChunkMeshUpdateEvent>,
    mut mesh_query: Query<(Entity, &terrain_components::ChunkMesh)>,
    texture_manager: ResMut<terrain_util::TextureManager>,
) {
    for event in chunk_mesh_update_events.read() {
        info!("Received chunk mesh update event for chunk {:?}", event.position);
        let chunk_option = chunk_manager.get_chunk(event.position);
        match chunk_option {
            Some(chunk) => {
                for (entity, chunk_mesh) in mesh_query.iter_mut() {
                    if lib::Chunk::key_eq_pos(chunk_mesh.key, chunk.position) {
                        commands.entity(entity).despawn();
                    }
                }
                add_chunk_objects(
                    &mut commands,
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    chunk,
                    &texture_manager,
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
    chunk: &lib::Chunk,
    texture_manager: &terrain_util::TextureManager,
) {
    if let Some(mesh) = create_chunk_mesh(chunk, texture_manager) {
        let texture_handle = obtain_texture_handle(asset_server).clone();
        let material = create_chunk_material(texture_handle, &mut ResMut::reborrow(materials));
        spawn_chunk(
            commands,
            &mut ResMut::reborrow(meshes),
            material,
            mesh,
            chunk,
        );
    }
}

fn create_chunk_mesh(
    chunk: &lib::Chunk,
    texture_manager: &terrain_util::TextureManager,
) -> Option<Mesh> {
    terrain_util::create_chunk_mesh(chunk, texture_manager)
}

fn create_chunk_material(
    texture_handle: Handle<Image>,
    materials: &mut Mut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        perceptual_roughness: 0.5,
        reflectance: 0.0,
        unlit: false,
        specular_transmission: 0.0,
        base_color_texture: Some(texture_handle),
        ..default()
    })
}

fn obtain_texture_handle(asset_server: &Res<AssetServer>) -> Handle<Image> {
    asset_server.load("textures/texture_atlas.png")
}

fn spawn_chunk(
    commands: &mut Commands,
    meshes: &mut Mut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
    mesh: Mesh,
    chunk: &lib::Chunk,
) {
    let transform = Transform::from_xyz(
        chunk.position.x * CHUNK_SIZE as f32,
        chunk.position.y * CHUNK_SIZE as f32,
        chunk.position.z * CHUNK_SIZE as f32,
    );

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
