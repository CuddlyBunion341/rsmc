use bevy::tasks::{futures_lite::future, AsyncComputeTaskPool};
use terrain_resources::{FutureChunkMesh, MeshTask, MeshType, MesherTasks, RenderMaterials};

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
    let render_distance = Vec3::new(8.0, 4.0, 8.0);

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
    chunk_manager: ResMut<ChunkManager>,
    mut chunk_mesh_update_events: EventReader<terrain_events::ChunkMeshUpdateEvent>,
    mut mesh_query: Query<(Entity, &terrain_components::ChunkMesh)>,
    texture_manager: ResMut<terrain_util::TextureManager>,
    mut tasks: ResMut<MesherTasks>,
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
                tasks.task_list.push(FutureChunkMesh {
                    position: chunk.position,
                    mesh_task: create_cube_mesher_task(chunk, &texture_manager),
                    mesh_type: terrain_resources::MeshType::Solid,
                });
                tasks.task_list.push(FutureChunkMesh {
                    position: chunk.position,
                    mesh_task: create_cross_mesher_task(chunk, &texture_manager),
                    mesh_type: terrain_resources::MeshType::Transparent,
                });
            }
            None => {
                println!("No chunk found");
            }
        }
    }
}

fn create_cube_mesher_task(
    chunk: &Chunk,
    texture_manager: &terrain_util::TextureManager,
) -> MeshTask {
    let task_pool = AsyncComputeTaskPool::get();

    let chunk = *chunk;
    let texture_manager = texture_manager.clone();

    MeshTask(task_pool.spawn(async move { terrain_util::create_cube_mesh_for_chunk(&chunk, &texture_manager) }))
}

fn create_cross_mesher_task(
    chunk: &Chunk,
    texture_manager: &terrain_util::TextureManager,
) -> MeshTask {
    let task_pool = AsyncComputeTaskPool::get();

    let chunk = *chunk;
    let texture_manager = texture_manager.clone();

    MeshTask(task_pool .spawn(async move { terrain_util::create_cross_mesh_for_chunk(&chunk, &texture_manager) }))
}

pub fn handle_chunk_tasks_system(
    mut commands: Commands,
    materials: Res<RenderMaterials>,
    mut tasks: ResMut<MesherTasks>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut next_poll_indicies: Vec<usize> = Vec::new();
    tasks
        .task_list
        .iter_mut()
        .enumerate()
        .for_each(|(index, future_chunk)| {
            let chunk_position = future_chunk.position;
            let task_result =
                bevy::tasks::block_on(future::poll_once(&mut future_chunk.mesh_task.0));
            if task_result.is_none() {
                // Check next poll
                next_poll_indicies.push(index);
                return;
            }
            let mesh_option = task_result.unwrap();
            if mesh_option.is_none() {
                return;
            }
            let mesh = mesh_option.expect("Mesh exists");
            let mesh_handle = meshes.add(mesh);

            let material_handle = match future_chunk.mesh_type {
                MeshType::Solid => materials.chunk_material.clone(),
                MeshType::Transparent => materials.transparent_material.clone(),
            };

            commands.spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(material_handle.expect("Material exists")),
                Transform::from_xyz(
                    chunk_position.x * CHUNK_SIZE as f32,
                    chunk_position.y * CHUNK_SIZE as f32,
                    chunk_position.z * CHUNK_SIZE as f32,
                ),
                terrain_components::ChunkMesh {
                    key: [
                        chunk_position.x as i32,
                        chunk_position.y as i32,
                        chunk_position.z as i32,
                    ],
                },
            ));
        });

    let mut index = 0;
    tasks.task_list.retain(|_| {
        let contains = next_poll_indicies.contains(&index);
        index += 1;
        contains
    })
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
