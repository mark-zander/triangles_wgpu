use triangles_wgpu::run;

mod cube;
mod grid;

fn main() {
    // println!("{:#?}", cube::make_cube());
    // println!("{:#?}", grid::make_zero());
    pollster::block_on(run());
}
