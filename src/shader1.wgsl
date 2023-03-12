// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

// Color table texture
@group(1) @binding(0)
var tc_diffuse: texture_1d<f32>;
@group(1)@binding(1)
var sc_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    var z: f32 = model.position.z;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.color = vec4<f32>(z, z, z, z);
    return out;
}

// Fragment shader

// Wire frame texture
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // return textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    let tex: vec4<f32> = 
        textureSample(t_diffuse, s_diffuse, in.tex_coords);
    return tex * tex.a + in.color * (1.0 - tex.a);
}