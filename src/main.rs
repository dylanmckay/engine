
#![feature(float_extras)]

extern crate libc;
extern crate gl as libgl;

pub mod num;
pub mod math;
pub mod color;
pub mod geom;
pub mod gfx;

const BACKGROUND: color::NormalizedRGBA = color::NormalizedRGBA(0.46,0.62,0.8,1.0);
const OBJ_PATH: &'static str = "/home/dylan/Desktop/model.obj";
const VERTEX_SHADER: &'static str = include_str!("../res/basic_vertex.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("../res/basic_fragment.glsl");

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
    use std;

    let file = std::fs::File::open(OBJ_PATH).unwrap();
    let mesh_data: geom::mesh::Data<Index,Vertex> = geom::formats::Wavefront::load(file);

    let backend = gfx::gl::backends::glfw::Backend::new();
    let mut device = gfx::gl::Device::new(backend);

    // compile and link the program
    let program = {
        let sources = [(gfx::gl::shader::Kind::Vertex, VERTEX_SHADER),
                       (gfx::gl::shader::Kind::Fragment, FRAGMENT_SHADER)];

        let shaders = sources.iter().map(|&(kind,source)| {
            match gfx::gl::shader::Shader::compile(kind,source) {
                Ok(shader) => shader,
                Err(msg) => { panic!(format!("failed to compile {} shader: {}", kind, msg)); },
            }
        });

        gfx::gl::shader::Program::link(shaders).unwrap()
    };

    let mesh = device.load_mesh_data(&mesh_data);

    while device.is_open() {
        device.run();

        let mut canvas = device.begin();
        canvas.set_background(BACKGROUND);

        canvas.clear();
        canvas.draw_mesh(&mesh, &program);

        device.end();

    }
}
