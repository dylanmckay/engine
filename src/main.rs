
#![feature(float_extras,concat_idents)]

extern crate libc;
extern crate gl as libgl;

pub mod num;
pub mod math;
pub mod color;
pub mod geom;
pub mod gfx;

const BACKGROUND: color::NormalizedRGBA = color::NormalizedRGBA(0.46,0.62,0.8,1.0);
const MODEL_DATA: &'static str = include_str!("../res/model.obj");
const VERTEX_SHADER: &'static str = include_str!("../res/vertex.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("../res/fragment.glsl");

#[repr(packed)]
pub struct Vertex {
    pub position: math::Vector3,
    pub normal: math::Vector3,
}

impl From<geom::formats::wavefront::Vertex> for Vertex
{
    fn from(val: geom::formats::wavefront::Vertex) -> Vertex {
        Vertex {
            position: val.position,
            normal: val.normal.expect("expected a normal"),
        }
    }
}

impl gfx::gl::Vertex for Vertex {
    fn formats() -> Vec<gfx::gl::vertex::FormatInfo> {
        use gfx::gl::vertex::Format;
        let info = <math::Vector3 as Format>::info();
        [info,info].iter().map(|&a|a).collect()
    }
}

type Index = u16;

pub struct Context
{
    device: gfx::gl::Device<gfx::gl::backends::glfw::Backend>,

    program: gfx::gl::Program,
    mesh: gfx::gl::mesh::Data,

    clock: f32,
}

impl Context
{
    pub fn new() -> Self {
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

        device.set_title("Engine Test");
        device.set_culling_mode(gfx::CullingMode::Back);

        let mesh = device.load_mesh_data(&mesh_data);

        let light_pos = math::Vector3(0.886,1.0,0.);
        program.uniform("lightPosition").set(light_pos);

        Context {
            device: device,
            program: program,
            mesh: mesh,
            clock: 0.0,
        }
    }

    pub fn is_open(&self) -> bool { self.device.is_open() }
    pub fn run(&mut self) {
        use gfx::Viewport;

        self.device.run();

        let (v1,v2) = self.device.area().split_half(math::Axis2::Vertical);
        let (v3,v4) = v2.split_half(math::Axis2::Horizontal);
        let mut canvas1 = v1.begin();
        let mut canvas2 = v3.begin();
        let mut canvas3 = v4.begin();

        self.render(&mut canvas1);
        self.render(&mut canvas2);
        self.render(&mut canvas3);

        self.device.end();

        self.clock += 0.05;
    }

    fn render(&self, canvas: &mut gfx::gl::Canvas) {
        use math::Matrix;

        canvas.set_background(BACKGROUND);

        canvas.clear();

        let y = self.clock.sin();
        let transform = geom::Transform3::identity()
//                        .scale(math::Vector3(y,y,y))
//                      .translate(math::Vector3(0.0,y,0.0));
                        .rotate(math::Vector3(y*0.2,y,0.0));



        self.program.uniform("worldTransform").set(transform);

        canvas.draw_mesh(&self.mesh, &self.program);


    }
}


fn main() {
    let mut context = Context::new();

    while context.is_open() {
        context.run();
    }
}
