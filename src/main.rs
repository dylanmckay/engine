
#![feature(float_extras)]

pub mod num;
pub mod math;
pub mod color;
pub mod geom;
pub mod gfx;

const BACKGROUND: color::NormalizedRGBA = color::NormalizedRGBA(0.46,0.62,0.8,1.0);

const OBJ_PATH: &'static str = "/home/dylan/Desktop/model.obj";

type Vertex = math::Vector3;
type Index = u16;

impl From<geom::formats::wavefront::Vertex> for Vertex
{
    fn from(vert: geom::formats::wavefront::Vertex) -> Vertex {
        vert.position
    }
}

fn main() {
    use geom::formats::Format;

    let file = std::fs::File::open(OBJ_PATH).unwrap();
    let mesh: geom::Mesh<Index,Vertex> = geom::formats::Wavefront::load(file);
/*
    let backend = gfx::gl::backends::glfw::Backend::new();
    let mut device = gfx::gl::Device::new(backend);

    while device.is_open() {
        device.run();

        let mut canvas = device.begin();
        canvas.set_background(BACKGROUND);

        canvas.clear();

        device.end();

    }*/
    println!("success!");
}
