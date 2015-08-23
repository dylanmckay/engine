
#![feature(float_extras)]

extern crate libc;
extern crate gl as libgl;

pub mod num;
pub mod math;
pub mod color;
pub mod geom;
pub mod gfx;

const BACKGROUND: color::NormalizedRGBA = color::NormalizedRGBA(0.46,0.62,0.8,1.0);
const MODEL_DATA: &'static str = include_str!("../res/model.obj");
const VERTEX_SHADER: &'static str = include_str!("../res/basic_vertex.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("../res/basic_fragment.glsl");

type Vertex = math::Vector3;
type Index = u16;


fn main() {
    use geom::formats::Format;
    use std;

    let mesh_cursor = std::io::Cursor::new(MODEL_DATA.as_bytes());
    let mesh_data: geom::mesh::Data<Index,Vertex> = geom::formats::Wavefront::load(mesh_cursor);

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
    let mut clock = 0.0f32;

    while device.is_open() {
        device.run();

        let mut canvas = device.begin();
        canvas.set_background(BACKGROUND);

        canvas.clear();

        let y = clock.sin();
        program.uniform("origin").set((0.,y,0.0,0.0));

        canvas.draw_mesh(&mesh, &program);

        device.end();

        clock += 0.05;
    }
}
