
use gfx;
use gfx::gl::gl::{self,types};
use gfx::gl::gl::types::*;
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

            let piece_formats = buffer.piece_formats;
            let piece_count = piece_formats.len();

            unsafe {
                for (i,piece_format) in piece_formats.iter().enumerate() {
                    let component_count = piece_format.component_count;

                    // TODO: check this somewhere else, return result
                    assert!(component_count > 0 && component_count <= 4,
                            "OpenGL only supports vertices with 1..4 components");


                    gl::EnableVertexAttribArray(i as GLuint);
                    gl::VertexAttribPointer(i as GLuint, component_count as GLint,
                                            piece_format.component_type,
                                            gl::FALSE, 0, 0 as *const c_void);

                }

                gl::DrawElements(gl::TRIANGLES, buffer.index_count as GLint,
                                 buffer.index_type, ptr::null());


                // disable the arrays
                for i in 0..piece_count {
                    gl::DisableVertexAttribArray(i as GLuint);
                }
            }

            buffer.unbind_indices();
            buffer.unbind_vertices();
        }

        program.disable();
    }
}

