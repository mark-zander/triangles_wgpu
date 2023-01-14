use std::f32::consts;
// use core::f32;

pub fn make_grid(xsize: u32, ysize: u32,
        xmin: f32, ymin: f32, xmax: f32, ymax: f32,
        ustep: f32, vstep: f32, fz: fn(f32, f32) -> f32)
        -> (Vec<Vertex>, Vec<u16>) {

    let xstep = (xmax - xmin) / (xsize - 1) as f32;
    let ystep = (ymax - ymin) / (ysize - 1) as f32;
    println!("xstep = {}, ystep = {}", xstep, ystep);
    let mut vertexes: Vec<Vertex> = Vec::new();
    for iy in 0..ysize {
        let fy = iy as f32;
        for ix in 0..xsize {
            let fx = ix as f32;
            let (x, y) = (xmin + fx * xstep, ymin + fy * ystep);
            let (u, v) = (0.0 + fx * ustep, 1.0 - fy * vstep);
            vertexes.push(Vertex {
                position: [x, y, fz(x, y)],
                tex_coord: [u, v]
            });
        }
    }

    let mut indexes: Vec<u16> = Vec::new();

    for iy in 0..ysize - 1 {
        let y = iy * xsize;
        let y1 = (iy + 1) * xsize;
        for ix in 0..xsize - 1 {
            indexes.push((ix + y) as u16);
            indexes.push(((ix + 1) + y1) as u16);
            indexes.push((ix + y1) as u16);
            indexes.push((ix + y) as u16);
            indexes.push(((ix + 1) + y) as u16);
            indexes.push(((ix + 1) + y1) as u16);
        }
    }

    (vertexes.to_vec(), indexes.to_vec())
}

pub fn make_zero() -> (Vec<Vertex>, Vec<u16>) {
    fn zero(_x: f32, _y: f32) -> f32 { 0.0 }
    make_grid(11, 11, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, zero)
}

pub fn sinc(x: f32, y: f32) -> f32 {
    let scale = 2.0;
    let d = scale * consts::PI * f32::sqrt(x * x + y * y);
    if d == 0.0 { 1.0 } else { f32::sin(d) / d }
}

pub fn make_sinc() -> (Vec<Vertex>, Vec<u16>) {
    make_grid(21, 21, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, sinc)
}

pub fn gauss(sigma: f32, mu: f32, x: f32, y: f32) -> f32 {
    let r = f32::sqrt(x * x + y * y);
    f32::exp(-(r - mu).powi(2)) / (2. * sigma * sigma)
        / (sigma * f32::sqrt(2. * consts::PI))
}

// pub fn make_gauss() -> (Vec<Vertex>, Vec<u32>) {
//     make_grid(50, 50, , 0.02, 1, 1, gauss(0.1, 0))
// }

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coord: [f32; 2],
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
