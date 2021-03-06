
use libgl;
use gl::{self,Viewport,Program};
use libgl::types::*;
use color::NormalizedRGBA;
use libc::c_void;
use std::ptr;

pub struct Canvas
{
    viewport: Viewport,
}

impl Canvas
{
    pub fn new(viewport: Viewport) -> Self {
        Canvas {
            viewport: viewport,
        }
    }

    pub fn set_background(&mut self, NormalizedRGBA(r,g,b,a): NormalizedRGBA) {
        unsafe {
            libgl::ClearColor(r,g,b,a)
        }
    }

    pub fn clear(&self) {
        self.pre_render();
        unsafe {
            libgl::Clear(libgl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_mesh(&self, mesh: &gl::mesh::Data, program: &Program) {

        self.pre_render();
        program.enable();

        for buffer in mesh.buffers() {
            buffer.bind_vertices();
            buffer.bind_indices();

            // Calculates the total size of the vertex.
            let vertex_size = buffer.formats.iter().fold(0, |a,f|a+f.total_size()) as GLsizei;

            unsafe {
                let mut cur_piece_offset = 0;

                for (i,format) in buffer.formats.iter().enumerate() {
                    let component_count = format.component_count;
                    let piece_size = format.total_size();

                    // TODO: check this somewhere else, return result
                    assert!(component_count > 0 && component_count <= 4,
                            "OpenGL only supports vertices with 1..4 components");


                    // Tell OpenGL about the current piece of the vertex.
                    libgl::EnableVertexAttribArray(i as GLuint);
                    libgl::VertexAttribPointer(i as GLuint, component_count as GLint,
                                            format.component_type,
                                            libgl::FALSE, vertex_size, cur_piece_offset as *const c_void);

                    cur_piece_offset += piece_size;
                }

                libgl::DrawElements(libgl::TRIANGLES, buffer.index_count() as GLint,
                                 buffer.index_type(), ptr::null());


                // disable the arrays
                for i in 0..buffer.formats.len() {
                    libgl::DisableVertexAttribArray(i as GLuint);
                }
            }

            buffer.unbind_indices();
            buffer.unbind_vertices();
        }

        program.disable();
    }

    fn pre_render(&self) {
        use Viewport;

        let (x,y) = self.viewport.top_left();
        let (width,height) = self.viewport.dimensions();
        unsafe {
            libgl::Viewport(x as i32, y as i32,
                         width as i32, height as i32);

            // TODO: Move this call somewhere more suitable
            libgl::Enable(libgl::SCISSOR_TEST);

            libgl::Scissor(x as i32, y as i32,
                        width as i32, height as i32);
        }
    }

    /// Gets the viewport associated with the canvas.
    pub fn viewport<'a>(&'a self) -> &'a Viewport {
        &self.viewport
    }
}

