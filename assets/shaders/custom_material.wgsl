// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let speed = 2.0;
    let distance_to_center = distance(in.uv, vec2<f32>(0.5)) * 1.4;
    let u = in.uv[0];
    let v = in.uv[1];

    return vec4<f32>(u, v, 1.0, 1.0);
}

