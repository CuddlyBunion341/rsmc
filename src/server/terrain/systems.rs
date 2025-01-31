use bevy::ui::Node;
use bevy_flair::style::components::NodeStyleSheet;

use crate::prelude::*;

pub fn setup_world_system(
    mut chunk_manager: ResMut<ChunkManager>,
    params: Res<TerrainGeneratorParams>,
) {
    let generator = terrain_util::generator::Generator::new(
        0,
        TerrainGeneratorParams {
            height_params: params.height_params,
            density_params: params.density_params,
        },
    );

    let render_distance = Vec3::new(12.0, 2.0, 12.0);

    info!("Generating chunks");

    let mut chunks = ChunkManager::instantiate_chunks(Vec3::ZERO, render_distance);

    chunks.par_iter_mut().for_each(|chunk| {
        info!("Generating chunk at {:?}", chunk.position);
        generator.generate_chunk(chunk);
    });

    chunk_manager.insert_chunks(chunks);
}

// visualizer

struct NoiseImageNode {}

pub fn setup_visualizer_system(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
) {
    commands.spawn((
            Node::default(),
            Name::new("visualizer"),
            NodeStyleSheet::new(asset_server.load("visualizer.css")),
    )).with_children(|parent| {
        parent.spawn((
                Node::default(),
                ImageNode::new(),
                Name::new("noise_image"),
        )
    })
}
