use terrain_resources::Mesher;
use terrain_util::{
    client_block::block_properties, get_cross_block_positions, instance_mesh_for_repr,
};

use crate::{
    materials::{
        create_chunk_material, create_custom_material, create_transparent_material, CustomMaterial,
    },
    prelude::*,
};

pub fn populate_mesher_meshes(
    mut mesher: ResMut<Mesher>,
    mut meshes: ResMut<Assets<Mesh>>,
    texture_manager: ResMut<terrain_util::TextureManager>,
) {
    BlockId::values().iter().for_each(|block_id| {
        let mesh_repr = block_properties(*block_id).mesh_representation;
        let mesh = instance_mesh_for_repr(mesh_repr.clone(), &texture_manager);
        if let Some(mesh) = mesh {
            let handle = meshes.add(mesh);
            mesher.mesh_handles.insert(mesh_repr, handle);
        }
    });
}

pub fn prepare_mesher_materials(
    mut mesher: ResMut<Mesher>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = obtain_texture_handle(&asset_server).clone();
    let material_handle = create_transparent_material(texture_handle, materials);
    mesher.transparent_material_handle = Some(material_handle);
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

#[allow(clippy::too_many_arguments)]
pub fn handle_chunk_mesh_update_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    chunk_manager: ResMut<ChunkManager>,
    mut chunk_mesh_update_events: EventReader<terrain_events::ChunkMeshUpdateEvent>,
    mut mesh_query: Query<(Entity, &terrain_components::ChunkMesh)>,
    texture_manager: ResMut<terrain_util::TextureManager>,
    mesher: Res<Mesher>,
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
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    chunk,
                    &texture_manager,
                );
                add_cross_objects(&mut commands, chunk, &mesher);
            }
            None => {
                println!("No chunk found");
            }
        }
    }
}

fn add_chunk_objects(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<CustomMaterial>>,
    chunk: &Chunk,
    texture_manager: &terrain_util::TextureManager,
) {
    if let Some(mesh) = create_chunk_mesh(chunk, texture_manager) {
        let texture_handle = obtain_texture_handle(&asset_server).clone();
        let material = create_custom_material(asset_server, texture_handle, materials);
        spawn_chunk(commands, meshes, material, mesh, chunk);
    }
}

fn add_cross_objects(commands: &mut Commands, chunk: &Chunk, mesher: &Mesher) {
    let values = get_cross_block_positions(chunk);
    for (mesh_repr, positions) in values {
        let mesh_handle = mesher
            .mesh_handles
            .get(&mesh_repr)
            .expect("Handle is not yet populated");
        let material_handle = mesher
            .transparent_material_handle
            .clone()
            .expect("Material has not yet been set");

        for position in positions {
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(
                    chunk.position.x * CHUNK_SIZE as f32 + position.x,
                    chunk.position.y * CHUNK_SIZE as f32 + position.y,
                    chunk.position.z * CHUNK_SIZE as f32 + position.z,
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
}

fn create_chunk_mesh(
    chunk: &Chunk,
    texture_manager: &terrain_util::TextureManager,
) -> Option<Mesh> {
    terrain_util::create_chunk_mesh(chunk, texture_manager)
}

fn obtain_texture_handle(asset_server: &Res<AssetServer>) -> Handle<Image> {
    asset_server.load("textures/texture_atlas.png")
}

fn spawn_chunk(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Handle<CustomMaterial>,
    mesh: Mesh,
    chunk: &Chunk,
) {
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
    ));
}

pub fn handle_terrain_regeneration_events(
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
