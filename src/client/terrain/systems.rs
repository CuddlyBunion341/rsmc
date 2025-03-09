use terrain_resources::RenderMaterials;
use terrain_util::create_cross_mesh_for_chunk;

use crate::prelude::*;

pub fn prepare_mesher_materials_system(
    mut render_materials: ResMut<RenderMaterials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = obtain_texture_handle(&asset_server);

    let material = create_transparent_material(texture_handle.clone());
    render_materials.transparent_material = Some(materials.add(material));

    let material = create_chunk_material(texture_handle);
    render_materials.chunk_material = Some(materials.add(material));
}

pub fn generate_simple_ground_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let mesh = Cuboid::new(64.0, 1.0, 64.0);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::srgba(1.0, 0.0, 1.0, 1.0))),
        Name::new("Simple Ground Plane"),
    ));
}

pub fn prepare_spawn_area_system(mut client: ResMut<RenetClient>) {
    info!("Sending chunk requests for spawn area");

    let chunks = ChunkManager::instantiate_chunks(Vec3::ZERO, Vec3::ONE);

    let positions: Vec<Vec3> = chunks.into_iter().map(|chunk| chunk.position).collect();
    let message = bincode::serialize(&NetworkingMessage::ChunkBatchRequest(positions));
    info!("requesting world");
    client.send_message(DefaultChannel::ReliableUnordered, message.unwrap());
}

pub fn generate_world_system(
    mut client: ResMut<RenetClient>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let render_distance = Vec3::new(4.0, 4.0, 4.0);

    info!("Sending chunk requests for chunks");

    let chunks = chunk_manager.instantiate_new_chunks(Vec3::new(0.0, 0.0, 0.0), render_distance);

    let positions: Vec<Vec3> = chunks.into_iter().map(|chunk| chunk.position).collect();

    let batched_positions = positions.chunks(16);
    assert!(batched_positions.len() > 0, "Batched positions is empty");

    batched_positions.enumerate().for_each(|(index, batch)| {
        let request_positions = batch.to_vec();
        info!(
            "Sending chunk batch request for {:?}",
            request_positions.len()
        );
        let message = bincode::serialize(&NetworkingMessage::ChunkBatchRequest(request_positions));
        info!("requesting chunks #{}", index);
        client.send_message(DefaultChannel::ReliableUnordered, message.unwrap());
    });
}

pub fn handle_chunk_mesh_update_events_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    chunk_manager: ResMut<ChunkManager>,
    mut chunk_mesh_update_events: EventReader<terrain_events::ChunkMeshUpdateEvent>,
    mut mesh_query: Query<(Entity, &terrain_components::ChunkMesh)>,
    texture_manager: ResMut<terrain_util::TextureManager>,
    materials: Res<RenderMaterials>,
) {
    for event in chunk_mesh_update_events.read() {
        info!(
            "Received chunk mesh update event for chunk {:?}",
            event.position
        );
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
                    &mut meshes,
                    chunk,
                    &texture_manager,
                    &materials,
                );
                add_cross_objects(
                    &mut commands,
                    chunk,
                    &materials,
                    &texture_manager,
                    &mut meshes,
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
    meshes: &mut ResMut<Assets<Mesh>>,
    chunk: &Chunk,
    texture_manager: &terrain_util::TextureManager,
    materials: &RenderMaterials,
) {
    if let Some(mesh) = terrain_util::create_chunk_mesh(chunk, texture_manager) {
        let material = materials
            .chunk_material
            .clone()
            .expect("Chunk material is loaded");

        let meshes: &mut Mut<Assets<Mesh>> = &mut ResMut::reborrow(meshes);
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            Transform::from_xyz(
                chunk.position.x * CHUNK_SIZE as f32,
                chunk.position.y * CHUNK_SIZE as f32,
                chunk.position.z * CHUNK_SIZE as f32,
            ),
            MeshMaterial3d(material),
            player_components::Raycastable,
            terrain_components::ChunkMesh {
                key: [
                    chunk.position.x as i32,
                    chunk.position.y as i32,
                    chunk.position.z as i32,
                ],
            },
            Name::from("Transparent Chunk Mesh"),
        ));
    }
}

fn add_cross_objects(
    commands: &mut Commands,
    chunk: &Chunk,
    materials: &RenderMaterials,
    texture_manager: &terrain_util::TextureManager,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    if let Some(mesh) = create_cross_mesh_for_chunk(chunk, texture_manager) {
        let mesh_handle = meshes.add(mesh);

        commands.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(
                materials
                    .transparent_material
                    .clone()
                    .expect("Transparent material exists"),
            ),
            Transform::from_xyz(
                chunk.position.x * CHUNK_SIZE as f32,
                chunk.position.y * CHUNK_SIZE as f32,
                chunk.position.z * CHUNK_SIZE as f32,
            ),
            terrain_components::ChunkMesh {
                key: [
                    chunk.position.x as i32,
                    chunk.position.y as i32,
                    chunk.position.z as i32,
                ],
            },
        ));
    }
}

fn create_transparent_material(texture_handle: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        perceptual_roughness: 1.0,
        double_sided: true,
        cull_mode: None,
        reflectance: 0.0,
        unlit: false,
        specular_transmission: 0.0,
        alpha_mode: AlphaMode::Mask(1.0),
        base_color_texture: Some(texture_handle),
        ..default()
    }
}

#[cfg(not(feature = "wireframe"))]
fn create_chunk_material(texture_handle: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        perceptual_roughness: 0.5,
        reflectance: 0.0,
        unlit: false,
        specular_transmission: 0.0,
        base_color_texture: Some(texture_handle),
        ..default()
    }
}

#[cfg(feature = "wireframe")]
fn create_chunk_material(_texture_handle: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
        alpha_mode: AlphaMode::Mask(0.5),
        ..default()
    }
}

fn obtain_texture_handle(asset_server: &Res<AssetServer>) -> Handle<Image> {
    asset_server.load("textures/texture_atlas.png")
}

pub fn handle_terrain_regeneration_events_system(
    mut client: ResMut<RenetClient>,
    mut world_regenerate_events: EventReader<terrain_events::WorldRegenerateEvent>,
    chunk_manager: ResMut<ChunkManager>,
) {
    for _ in world_regenerate_events.read() {
        info!("Rerequesting all chunks from server");
        let all_chunk_positions = chunk_manager.get_all_chunk_positions();
        let message =
            bincode::serialize(&NetworkingMessage::ChunkBatchRequest(all_chunk_positions));
        client.send_message(DefaultChannel::ReliableUnordered, message.unwrap());
    }
}
