use std::iter;
use crate::vertex::Vertex;

fn cube_data() -> (Vec<[i16; 3]>, Vec<[usize; 8]>, Vec<[[i16; 2]; 8]> ) {
    let vertexes = [
        [-1, -1, -1], // 0
        [ 1, -1, -1], // 1
        [-1,  1, -1], // 2
        [ 1,  1, -1], // 3
        [-1, -1,  1], // 4
        [ 1, -1,  1], // 5
        [-1,  1,  1], // 6
        [ 1,  1,  1], // 7

    ];

    let quad_strips = [
        [ 0, 2, 4, 6, 5, 7, 1, 3 ],
        [ 4, 5, 0, 1, 2, 3, 6, 7 ],
    ];

    let tex_strips = [
        [[0, 1], [0, 0], [1, 1], [1, 0], [2, 1], [2, 0], [3, 1], [3, 0]],
        [[0, 1], [0, 0], [1, 1], [1, 0], [2, 1], [2, 0], [3, 1], [3, 0]]
    ];

    (vertexes.to_vec(), quad_strips.to_vec(), tex_strips.to_vec())
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

fn vert(p:[i16; 3], t:[i16; 2]) -> Vertex {
    Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32,],
        tex_coord: [t[0] as f32, t[1] as f32,],
    }
}

pub fn make_cube() -> (Vec<Vertex>, Vec<u16>) {
    let (verts, strips, tex_strips) = cube_data();
    let mut vertexes: Vec<Vertex> = Vec::new();
    for i in 0..strips.len() {
        for j in 0..strips[i].len() {
            vertexes.push(vert(verts[strips[i][j]], tex_strips[i][j]));
        }
    }

    let mut indexes: Vec<u16> = Vec::new();

    let mut j: u16 = 0;
    for i in 0..strips.len() {
        let k = strips[i].len() as u16 + j;
        while j < k - 2 {
            indexes.push(j);
            indexes.push(j+3);
            indexes.push(j+1);
            indexes.push(j);
            indexes.push(j+2);
            indexes.push(j+3);
            j += 2;
        }
        j = k;
    }

    (vertexes.to_vec(), indexes.to_vec())
}

// tet
// [a,b,c], [a,b,d], [a,c,d], [b,c,d]
// reorder: [a,b,c], [b,c,d], [c,d,a], [d,a,b]
// triangle strip: [a,b,c,d,a,b]
