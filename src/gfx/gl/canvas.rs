
use gfx;
use gfx::gl::gl;
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

            // Calculates the total size of the vertex.
            let vertex_size = buffer.piece_formats.iter().fold(0, |a,f|a+f.total_size()) as GLsizei;

            unsafe {
                let mut cur_piece_offset = 0;

                for (i,piece_format) in buffer.piece_formats.iter().enumerate() {
                    let component_count = piece_format.component_count;
                    let piece_size = piece_format.total_size();

                    // TODO: check this somewhere else, return result
                    assert!(component_count > 0 && component_count <= 4,
                            "OpenGL only supports vertices with 1..4 components");


                    // Tell OpenGL about the current piece of the vertex.
                    gl::EnableVertexAttribArray(i as GLuint);
                    gl::VertexAttribPointer(i as GLuint, component_count as GLint,
                                            piece_format.component_type,
                                            gl::FALSE, vertex_size, cur_piece_offset as *const c_void);

                    cur_piece_offset += piece_size;
                }

                gl::DrawElements(gl::TRIANGLES, buffer.index_count as GLint,
                                 buffer.index_type, ptr::null());


                // disable the arrays
                for i in 0..buffer.piece_formats.len() {
                    gl::DisableVertexAttribArray(i as GLuint);
                }
            }

            buffer.unbind_indices();
            buffer.unbind_vertices();
        }

        program.disable();
    }
}

