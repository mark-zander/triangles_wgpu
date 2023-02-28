
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

// let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//     label: Some("Vertex Buffer"),
//     contents: bytemuck::cast_slice(&vertexes),
//     usage: wgpu::BufferUsages::VERTEX,
// });

// pub trait Attr {
//     type Type;
//     const format: wgpu::VertexFormat;
// }

// // struct Layout<T>

// pub struct Float32();
// impl Attr for Float32 {
//     type Type = f32;
//     const format: wgpu::VertexFormat = wgpu::VertexFormat::Float32;
//     type ShaderType = f32;
//     const size: wgpu::BufferAddress = mem::size_of::Type as wgpu::BufferAddress;
// }
// Float32
// Float32x2
// Float32x3
// Float32X4

// pub fn makeAttr<T>(location, offset) -> wgpu::VertexAttribute {
//     (wgpu::VertexAttribute {
//         offset: offset;
//         location: location;
//         format: T::format;
//     }, offset + T::size, location + 1)
// }
