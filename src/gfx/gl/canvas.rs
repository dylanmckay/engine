
use gfx;
use gfx::gl::gl;
use color::NormalizedRGBA;

pub struct Canvas;

impl Canvas
{
    pub fn set_background(&mut self, NormalizedRGBA(r,g,b,a): NormalizedRGBA) {
        unsafe {
            gl::ClearColor(r,g,b,a)
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_mesh(&self, mesh: &gfx::gl::mesh::Data, program: &gfx::gl::Program) {
        unsafe {
            gl::EnableVertexAttribArray(0);

        }
    }
}

