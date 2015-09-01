
#![feature(float_extras,concat_idents)]

extern crate libc;
extern crate gl as libgl;

pub mod num;
pub mod math;
pub mod color;
pub mod geom;
pub mod gfx;

const BACKGROUND: color::NormalizedRGBA = color::NormalizedRGBA(0.46,0.62,0.8,1.0);
const MODEL_DATA: &'static str = include_str!("../res/unit_cube.obj");
const VERTEX_SHADER: &'static str = include_str!("../res/vertex.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("../res/fragment.glsl");

const BLOCK_SIZE: f32 = 0.005;

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

    chunk: Chunk,
}

pub struct Chunk
{
    blocks: [[[Block; 15]; 15]; 15],
}

impl Chunk
{
    pub fn from_fn<F>(f: F) -> Self
        where F: Fn(u32,u32,u32) -> Block {
        let mut blocks = [[[Block::Empty; 15]; 15]; 15];

        for x in 0..15 {
            for y in 0..15 {
                for z in 0..15 {
                    blocks[x as usize][y as usize][z as usize] = f(x,y,z);
                }
            }
        }

        Chunk {
            blocks: blocks,
        }

    }

    pub fn set(&mut self, (x,y,z): (u32,u32,u32), block: Block) {
        self.blocks[x as usize][y as usize][z as usize] = block;
    }

    pub fn get(&mut self, (x,y,z): (u32,u32,u32)) -> Block {
        self.blocks[x as usize][y as usize][z as usize]
    }

    pub fn render(&self, context: &Context, canvas: &mut gfx::gl::Canvas) {
        use math::Matrix;
        for (xi,a) in self.blocks.iter().enumerate() {
            for (yi,b) in a.iter().enumerate() {
                for (zi,block) in b.iter().enumerate() {

                    match *block {
                        Block::Square((r,g,b)) => {
                            let x = xi as f32 * BLOCK_SIZE;
                            let y = yi as f32 * BLOCK_SIZE;
                            let z = zi as f32 * BLOCK_SIZE;
                            let transform = geom::Transform3::identity()
                                           .translate(math::Vector3(x,y,z));

                            context.program.uniform("modelTransform").set(transform);

                            canvas.draw_mesh(&context.mesh, &context.program);
                        },
                        Block::Empty => { },
                    }
                }
            }
        }
    }
}

#[derive(Copy,Clone)]
pub enum Block
{
    Square((f32,f32,f32)),
    Empty,
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

        let chunk = Chunk::from_fn(|x,y,z| {
            if x%2 == 0 {
                Block::Square( (1.0/x as f32, 1.0/y as f32, 1.0/z as f32) )
            } else {
                Block::Empty
            }
        });

        Context {
            device: device,
            program: program,
            mesh: mesh,
            clock: 0.0,
            chunk: chunk,
        }
    }

    pub fn is_open(&self) -> bool { self.device.is_open() }
    pub fn run(&mut self) {
        use gfx::Viewport;

        self.device.run();

        let mut canvas = self.device.begin();
        self.render(&mut canvas);

        self.device.end();

        self.clock += 0.05;
    }

    fn render(&self, canvas: &mut gfx::gl::Canvas) {
        use math::Matrix;
        use gfx::Viewport;

        canvas.set_background(BACKGROUND);

        canvas.clear();

        let transform = geom::Transform3::identity()
                    .scale(math::Vector3(0.5,0.5,0.5));
        //                .translate(math::Vector3(0.0,y,0.0));
         //               .rotate(math::Vector3(y*0.2,y,0.0));



        let projection = geom::Transform3::perspective(45.0, 0.0, 15.0, canvas.viewport().aspect());
        self.program.uniform("worldTransform").set(projection);

        self.chunk.render(self, canvas);
    }
}


fn main() {
    let mut context = Context::new();

    while context.is_open() {
        context.run();
    }
}
