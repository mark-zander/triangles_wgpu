use std::f32::consts;
use crate::vertex::Vertex;

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
        let y = iy * (steps + 1);
        let y1 = (iy + 1) * (steps + 1);
        for ix in 0..steps {
            // quad_index(&mut indexes, ix as u16, y as u16, (ix+1) as u16, y1 as u16);
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

pub fn quad_index(indexes: &mut Vec<u16>, x: u16, y: u16, x1: u16, y1: u16) {
    indexes.push(x + y);
    indexes.push(x1 + y);
    indexes.push(x + y1);

    indexes.push(x + y1);
    indexes.push(x1 + y);
    indexes.push(x1 + y1);    
}

// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct Vertex {
//     position: [f32; 3],
//     tex_coord: [f32; 2],
// }

// impl Vertex {
//     pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
//         use std::mem;
//         wgpu::VertexBufferLayout {
//             array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &[
//                 wgpu::VertexAttribute {
//                     offset: 0,
//                     shader_location: 0,
//                     format: wgpu::VertexFormat::Float32x3,
//                 },
//                 wgpu::VertexAttribute {
//                     offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
//                     shader_location: 1,
//                     format: wgpu::VertexFormat::Float32x2,
//                 },
//             ],
//         }
//     }
// }

pub fn line(steps: u32, xmin: f32, xmax: f32, ymin: f32, ymax: f32)
        -> Vec<(f32, f32)> {
    let mut pts: Vec<(f32, f32)> = Vec::new();
    let xspace = (xmax - xmin) / steps as f32;
    let yspace = (ymax - ymin) / steps as f32;
    for i in 0..=steps {
        let x = xmin + xspace * i as f32;
        let y = ymin + yspace * i as f32;
        pts.push((x, y));
    }
    pts
}

pub fn cylinder()  -> (Vec<Vertex>, Vec<u16>) {
    surf_rot(8, line(8, 1.0, 1.0, -1.0, 1.0), 1.0, 1.0)
}

pub fn cone()  -> (Vec<Vertex>, Vec<u16>) {
    surf_rot(8, line(8, -1.0, 1.0, -1.0, 1.0), 1.0, 1.0)
}

pub fn arc(steps: u32, tmin: f32, tmax: f32, r: f32)
        -> Vec<(f32, f32)> {
    let mut pts: Vec<(f32, f32)> = Vec::new();
    let angle = (tmax - tmin) / steps as f32;
    for i in 0..=steps {
        let theta: f32 = tmin + angle * i as f32;
        pts.push((r * f32::cos(theta), r * f32::sin(theta)));
    }
    pts
}

pub fn torus()  -> (Vec<Vertex>, Vec<u16>) {
    let mut circle: Vec<(f32, f32)> = arc(8, 0.0, consts::TAU, 0.1);
    for i in 0..circle.len() { circle[i].0 += 0.5 }
    // println!("circle len = {}", circle.len());
    // println!("{:#?}", circle);
    surf_rot(8, circle, 1.0, 1.0)
}

pub fn sphere()  -> (Vec<Vertex>, Vec<u16>) {
    let pi_2 = consts::FRAC_PI_2;
    surf_rot(8, arc(8, -pi_2, pi_2, 1.0), 1.0, 1.0)
}

pub fn parabola(steps: u32, xmin: f32, xmax: f32) -> Vec<(f32, f32)> {
    let mut pts: Vec<(f32, f32)> = Vec::new();
    let xspace = (xmax - xmin) / steps as f32;
    for i in 0..=steps {
        let x = xmin + xspace * i as f32;
        let y = x * x as f32;
        pts.push((x, y));
    }
    pts
}

pub fn paraboloid()  -> (Vec<Vertex>, Vec<u16>) {
    surf_rot(8, parabola(8, 0.0, 1.0), 1.0, 1.0)
}

pub fn hyperbola(steps: u32, a: f32, b: f32, c: f32)
        -> Vec<(f32, f32)> {
    let mut pts: Vec<(f32, f32)> = Vec::with_capacity((steps + 1) as usize);
    let ymin = -1.0;
    let ymax = 1.0;
    let yspace = (ymax - ymin) / steps as f32;
    for i in 0..=steps {
        let y = ymin + yspace * i as f32;
        let xsq = -(a * y * y + 2.0 * b * y + c);
        if xsq >= 0.0 {
            pts.push((f32::sqrt(xsq), y));
        }
    }
    pts
}

pub fn hyperboloid(a: f32, b: f32, c: f32)  -> (Vec<Vertex>, Vec<u16>) {
    surf_rot(8, hyperbola(8, a, b, c), 1.0, 1.0)
}


