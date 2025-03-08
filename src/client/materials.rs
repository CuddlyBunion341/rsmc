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

pub fn create_custom_material(
    asset_server: Res<AssetServer>,
) -> CustomMaterial {
    CustomMaterial {
        color: LinearRgba::BLUE,
        color_texture: Some(asset_server.load("branding/icon.png")),
        alpha_mode: AlphaMode::Blend,
    }
}
