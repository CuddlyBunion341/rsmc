use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use crate::prelude::*;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

const SHADER_ASSET_PATH: &str = "shaders/custom_material.wgsl";

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

pub fn create_custom_material(asset_server: Res<AssetServer>) -> CustomMaterial {
    CustomMaterial {
        color: LinearRgba::BLUE,
        color_texture: Some(asset_server.load("branding/icon.png")),
        alpha_mode: AlphaMode::Blend,
    }
}

pub fn create_transparent_material(
    texture_handle: Handle<Image>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        perceptual_roughness: 1.0,
        double_sided: true,
        cull_mode: None,
        reflectance: 0.0,
        unlit: false,
        specular_transmission: 0.0,
        alpha_mode: AlphaMode::Mask(1.0),
        base_color_texture: Some(texture_handle),
        ..default()
    })
}

#[cfg(not(feature = "wireframe"))]
pub fn create_chunk_material(
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

#[cfg(feature = "wireframe")]
pub fn create_chunk_material(
    _texture_handle: Handle<Image>,
    materials: &mut Mut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
        alpha_mode: AlphaMode::Mask(0.5),
        ..default()
    })
}
