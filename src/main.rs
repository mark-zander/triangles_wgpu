use triangles_wgpu::run;
use triangles_wgpu::cli;

fn main() {

    pollster::block_on(run(cli::Args::new()));

}
