use std::f32::consts;
// use core::f32;

pub fn surf_rot(steps: i32, pts: Vec<(f32, f32)>,
        ustep: f32, vstep: f32)
        -> (Vec<Vertex>, Vec<u16>) {

    let theta = consts::TAU / steps as f32;
    // println!("xstep = {}, ystep = {}", xstep, ystep);
    let mut vertexes: Vec<Vertex> = Vec::new();
    for j in 0..pts.len() {
        let fj = j as f32;
        let (r, y) = pts[j];
        for i in 0..=steps {
            let fi = i as f32;
            let angle = theta * fi;
            let (x, z) = (r * f32::cos(angle), r * f32::sin(angle));
            let (u, v) = (0.0 + fi * ustep, 1.0 - fj * vstep);
            vertexes.push(Vertex {
                position: [x, y, z],
                tex_coord: [u, v]
            });
        }
    }

    let mut indexes: Vec<u16> = Vec::new();

    for iy in 0..pts.len() as i32 - 1 {
        let y = iy * steps;
        let y1 = (iy + 1) * steps;
        for ix in 0..steps {
            indexes.push((ix + y) as u16);
            indexes.push((ix + y1) as u16);
            indexes.push(((ix + 1) + y1) as u16);
            indexes.push(((ix + 1) + y) as u16);
            indexes.push((ix + y) as u16);
            indexes.push(((ix + 1) + y1) as u16);
        }
    }

    (vertexes.to_vec(), indexes.to_vec())
}


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

fn line(x: f32, ymin: f32, ymax: f32, steps: u32) -> Vec<(f32, f32)> {
    let mut pts: Vec<(f32, f32)> = Vec::new();
    let yspace = (ymax - ymin) / steps as f32;
    for i in 0..=steps {
        let y = ymin + yspace * i as f32;
        pts.push((x, y));
    }
    pts
}

pub fn cylinder()  -> (Vec<Vertex>, Vec<u16>) {
    surf_rot(6, line(1.0, -1.0, 1.0, 10), 1.0, 1.0)
}

fn arc(steps: u32, tmin: f32, tmax: f32, r: f32)
        -> Vec<(f32, f32)> {
    let mut pts: Vec<(f32, f32)> = Vec::new();
    let angle = (tmax - tmin) / steps as f32;
    for i in 0..=steps {
        let theta: f32 = tmin + angle * i as f32;
        pts.push((r * f32::cos(theta), r * f32::sin(theta)));
    }
    pts
}

pub fn sphere()  -> (Vec<Vertex>, Vec<u16>) {
    let pi_2 = consts::FRAC_PI_2;
    surf_rot(8, arc(8, -pi_2, pi_2, 1.0), 1.0, 1.0)
}
