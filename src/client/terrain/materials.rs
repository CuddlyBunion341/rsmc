use crate::prelude::*;
use bevy::{
    pbr::MaterialExtension,
    reflect::Reflect,
    render::render_resource::{AsBindGroup, ShaderRef},
};

const SHADER_ASSET_PATH: &str = "shaders/my_material.glsl";

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct MyExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    pub quantize_steps: u32,
}

impl MaterialExtension for MyExtension {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

pub fn create_base_material(
    _foo: Handle<Image>,
) -> ExtendedMaterial<StandardMaterial, MyExtension> {
    ExtendedMaterial {
        base: StandardMaterial {
            opaque_render_method: OpaqueRendererMethod::Deferred,
            perceptual_roughness: 0.5,
            reflectance: 0.0,
            unlit: false,
            specular_transmission: 0.0,
            // base_color_texture: Some(texture_handle),
            ..default()
        },
        extension: MyExtension { quantize_steps: 0 },
    }
}
