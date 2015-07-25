
use gfx;
use gfx::gl::gl::{self,types};
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

            let vertex_format = buffer.vertex_format;

            // TODO: check this somewhere else, return result
            assert!(vertex_format.component_count > 0 && vertex_format.component_count <= 4,
                    "OpenGL only supports vertices with 1..4 components");

            unsafe {
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(0, vertex_format.component_count as types::GLint,
                                        vertex_format.component_type,
                                        gl::FALSE, 0, 0 as *const c_void);

                gl::DrawElements(gl::TRIANGLES, buffer.index_count as types::GLint,
                                 buffer.index_type, ptr::null());
                gl::DisableVertexAttribArray(0);
            }

            buffer.unbind_indices();
            buffer.unbind_vertices();
        }

        program.disable();
    }
}

