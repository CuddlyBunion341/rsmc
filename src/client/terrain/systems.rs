use crate::prelude::*;

pub fn setup_world_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<terrain_resources::ChunkManager>,
    texture_manager: ResMut<terrain_util::TextureManager>,
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
            &texture_manager,
        );
    }

    chunk_manager.insert_chunks(chunks);
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
    chunk: &terrain_util::Chunk,
    texture_manager: &terrain_util::TextureManager,
) {
    if let Some(mesh) = create_chunk_mesh(chunk, texture_manager) {
        let material = create_chunk_material(asset_server, &mut ResMut::reborrow(materials));
        spawn_chunk(commands, &mut ResMut::reborrow(meshes), material, mesh, chunk);
    }
}

fn create_chunk_mesh(
    chunk: &terrain_util::Chunk,
    texture_manager: &terrain_util::TextureManager,
) -> Option<Mesh> {
    terrain_util::create_chunk_mesh(chunk, texture_manager)
}

fn create_chunk_material(
    asset_server: &Res<AssetServer>,
    materials: &mut Mut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    // use Mut instead of ResMut because of https://github.com/bevyengine/bevy/issues/11765
    let texture_handle: Handle<Image> = asset_server.load("textures/texture_atlas.png");
    materials.add(StandardMaterial {
        perceptual_roughness: 0.5,
        reflectance: 0.0,
        unlit: false,
        specular_transmission: 0.0,
        base_color_texture: Some(texture_handle.clone()),
        ..default()
    })
}

fn spawn_chunk(
    commands: &mut Commands,
    meshes: &mut Mut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
    mesh: Mesh,
    chunk: &terrain_util::Chunk,
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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::AssetPlugin;

    fn setup_app() -> App {
        let mut app = App::new();
        app
            .add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default());
        app
    }

    #[test]
    fn test_create_chunk_material() {
        let mut app = setup_app();
        let mut world = app.world;

        world.insert_resource(Assets::<StandardMaterial>::default());

        let asset_server = world.get_resource_ref::<AssetServer>().unwrap();
        let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();

        let material = create_chunk_material(&asset_server, &mut materials);
        assert!(materials.get(&material).is_some());
    }


    #[test]
    fn test_spawn_chunk() {
        let mut app = setup_app();
        let world = &mut app.world;
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, world);

        world.insert_resource(Assets::<StandardMaterial>::default());
        world.insert_resource(Assets::<Mesh>::default());

        let asset_server = world.get_resource_ref::<AssetServer>().unwrap();
        let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

        let chunk = terrain_util::Chunk::default();
        let mesh = create_chunk_mesh(&chunk, &terrain_util::TextureManager::default()).unwrap();
        let material = create_chunk_material(&asset_server, &mut materials);

        spawn_chunk(&mut commands, &mut meshes, material, mesh, &chunk);

        command_queue.apply(world);

        let query = world.query::<&terrain_components::ChunkMesh>();
        assert_eq!(query.iter(world).count(), 1);
    }
}
