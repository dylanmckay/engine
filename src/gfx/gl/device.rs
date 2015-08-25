
use gfx::{self,gl};
use libgl;
use geom;

pub struct Device<B: gl::Backend>
{
    backend: B,
}

impl<B: gl::Backend> Device<B>
{
    pub fn new(backend: B) -> Self {
        Device {
            backend: backend,
        }
    }

    pub fn run(&mut self) {
        self.backend.run()
    }

    pub fn is_open(&self) -> bool {
        self.backend.is_open()
    }

    pub fn begin(&self) -> gl::Canvas {
        gl::Canvas
    }

    pub fn end(&mut self) {
        self.backend.end()
    }

    /// Gets the dimensions in pixels.
    pub fn dimensions(&self) -> (u32,u32) {
        self.backend.dimensions()
    }

    /// Enables culling.
    pub fn enable_culling(&mut self) {
        unsafe {
            libgl::Enable(libgl::CULL_FACE);
            // Use clockwise vertices as front,
            libgl::FrontFace(libgl::CW);
        }
    }

    /// Disables culling.
    pub fn disable_culling(&mut self) {
        unsafe {
            libgl::Disable(libgl::CULL_FACE);
        }
    }

    /// Sets the culling mode.
    /// This enables culling if it is disabled.
    pub fn set_culling_mode(&mut self, mode: gfx::CullingMode) {
        let mode_enum = gl::util::culling_mode(mode);

        self.enable_culling();
        unsafe {
            libgl::CullFace(mode_enum);
        }
    }

    /// Loads mesh data.
    pub fn load_mesh_data<I,V>(&mut self, data: &geom::mesh::Data<I,V>)
        -> gl::mesh::Data
        where I: gl::Type, V: gl::Vertex {
        gl::mesh::Data::new().load(data, libgl::STATIC_DRAW)
    }

    /// Sets the title.
    /// This operation is not always supported. If this is the case, this
    /// function does nothing.
    pub fn set_title(&mut self, title: &str) {
        self.backend.set_title(title);
    }
}


