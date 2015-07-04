
use gfx::gl::{self};

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
}


