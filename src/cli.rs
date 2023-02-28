use clap::Parser;
use clap::ValueEnum;
use crate::geometry;
use crate::vertex;

#[derive(Parser,Default,Debug)]
#[clap(author="Author Name", version, about)]
/// Make 3D graphics using triangles in wgpu
struct Cli {
    /// Geometry to draw
    #[arg(value_enum, default_value_t=Geometry::Sphere)]
    geometry: Geometry,    
    // #[arg(value_enum, short, long, default_value_t=FrontFace::Ccw)]
    #[arg(value_enum, short, long, default_value_t=FrontFace::Ccw)]
    /// Face considered front for culling and stencil ops
    front_face: FrontFace,
    #[arg(value_enum, short, long)]
    /// Face culling mode
    cull_mode: Option<Face>,
    #[arg(value_enum, short, long, default_value_t=PolygonMode::Fill)]
    /// Controls the way each polygon is rasterized
    polygon_mode: PolygonMode,
}

impl Cli {
    fn front_face(&self) -> wgpu::FrontFace {
        match self.front_face {
            FrontFace::Ccw => wgpu::FrontFace::Ccw,
            FrontFace::Cw => wgpu::FrontFace::Cw,
        }
    }
    fn cull_mode(&self) -> Option<wgpu::Face> {
        match self.cull_mode {
            None => None,
            Some(Face::Front) => Some(wgpu::Face::Front),
            Some(Face::Back) => Some(wgpu::Face::Back)
        }
    }
    fn polygon_mode(&self) -> wgpu::PolygonMode {
        match self.polygon_mode {
            PolygonMode::Fill => wgpu::PolygonMode::Fill,
            PolygonMode::Line => wgpu::PolygonMode::Line,
            PolygonMode::Point => wgpu::PolygonMode::Point
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum FrontFace {
    #[default]
    Ccw,
    Cw
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Face {
    Front,
    Back
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum PolygonMode {
    #[default]
    Fill,
    Line,
    Point
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum Geometry {
    Cube,
    Grid,
    Sinc,
    Hp,
    Gauss,
    #[default]
    Sphere,
    Paraboloid,
    Hyperboloid1,
    Cone,
    Cylinder,
    Torus,
}

impl Geometry {
    pub fn make(&self) -> (Vec<vertex::Vertex>, Vec<u16>) {
        match &self {
            Geometry::Cube => geometry::cube::make_cube(),
            Geometry::Grid => geometry::grid::make_zero(),
            Geometry::Sinc => geometry::grid::make_sinc(),
            Geometry::Hp => geometry::grid::make_hp(),
            Geometry::Gauss => geometry::grid::make_gauss(),
            Geometry::Sphere =>
                geometry::surf_rot::hyperboloid(1.0, 0.0, -1.0),
            Geometry::Paraboloid =>
                geometry::surf_rot::hyperboloid(0.0, -0.5, 0.0),
            Geometry::Hyperboloid1 =>
                geometry::surf_rot::hyperboloid(-1.0, 0.0, -1.0),
            Geometry::Cone =>
                geometry::surf_rot::hyperboloid(-1.0, 0.0, 0.0),
            Geometry::Cylinder =>
                geometry::surf_rot::hyperboloid(0.0, 0.0, -1.0),
            Geometry::Torus => geometry::surf_rot::torus(),
        }
    }
}

pub struct Args {
    pub geometry: Geometry,
    pub front_face: wgpu::FrontFace,
    pub cull_mode: Option<wgpu::Face>,
    pub polygon_mode: wgpu::PolygonMode,
}

impl Args {
    pub fn new() -> Self {
        let cli = Cli::parse();
        println!("{:?}", cli);
        Self {
            geometry: cli.geometry,
            front_face: cli.front_face(),
            cull_mode: cli.cull_mode(),
            polygon_mode: cli.polygon_mode(),
        }
    }
}