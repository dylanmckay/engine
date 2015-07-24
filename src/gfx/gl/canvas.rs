
use gfx;
use gfx::gl::gl;
use color::NormalizedRGBA;
use libc::c_void;
use std::ptr;

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
        program.enable();

        for buffer in mesh.buffers() {
            buffer.bind_vertices();
            buffer.bind_indices();
            unsafe {
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, 0 as *const c_void);

                gl::DrawElements(gl::TRIANGLES, buffer.index_count as gl::types::GLsizei,
                                 gl::UNSIGNED_SHORT, ptr::null());
                gl::DisableVertexAttribArray(0);
            }

            buffer.unbind_indices();
            buffer.unbind_vertices();
        }

        program.disable();
    }
}

