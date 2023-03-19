// Vertex shader

struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct GreyScaleUniform {
    low: f32,
    high: f32,
}
@group(2) @binding(0)
var<uniform> greyscale: GreyScaleUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) grey: f32,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let z: f32 = model.position.z;
    let lo:f32 = -1.0;
    let hi = 1.0;
    // let grey: f32 = (clamp(z, grey.low, grey.high) - grey.low) /
    //     (grey.high - grey.low);
    out.grey = (clamp(z, lo, hi) - lo) / (hi - lo);
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    // out.color = vec4<f32>(z, z, z, z);
    return out;
}

// Fragment shader

// Wire frame texture
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

// Color table texture
@group(2) @binding(0)
var ctab_tex: texture_1d<f32>;
@group(2)@binding(1)
var ctab_samp: sampler;

// Both wire frame and color
@fragment
fn fs_both(in: VertexOutput) -> @location(0) vec4<f32> {
    // return textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    let tex: vec4<f32> = 
        textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let color: vec4<f32> =
        textureSample(ctab_tex, ctab_samp, in.grey);
    return tex * tex.a + color * (1.0 - tex.a);
}
// Only wire frame texture
@fragment
fn fs_texture(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // let tex: vec4<f32> = 
    //     textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // let color: vec4<f32> =
    //     textureSample(ctab_tex, ctab_samp, in.grey);
    // return tex * tex.a + color * (1.0 - tex.a);
}

// Only color
@fragment
fn fs_colors(in: VertexOutput) -> @location(0) vec4<f32> {
    // return textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    // let tex: vec4<f32> = 
    //     textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // let color: vec4<f32> =
    //     textureSample(ctab_tex, ctab_samp, in.grey);
    // return tex * tex.a + color * (1.0 - tex.a);
    return textureSample(ctab_tex, ctab_samp, in.grey);
}

// Hardware wire frame
@fragment
fn fs_wire(in: VertexOutput) -> @location(0) vec4<f32> {
    // return textureSample(t_diffuse, s_diffuse, in.tex_coords);
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    // let tex: vec4<f32> = 
    //     textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // let color: vec4<f32> =
    //     textureSample(ctab_tex, ctab_samp, in.grey);
    // return tex * tex.a + color * (1.0 - tex.a);
    // return textureSample(ctab_tex, ctab_samp, in.grey);
}

