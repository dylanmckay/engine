
use gfx::gl::gl;
use gfx::gl::gl::types::*;
use geom;
use libc::types::common::c95::c_void;
use std::mem;

pub struct Buffer
{
    buffer: GLuint,
}

impl Buffer
{
    pub fn from_buffer(buffer: GLuint) -> Buffer {
        Buffer {
            buffer: buffer,
        }
    }

    pub fn new() -> Self {
        let mut buffer = 0;

        unsafe {
            gl::GenBuffers(1, &mut buffer);
        }

        Buffer {
            buffer: buffer,
        }
    }

    pub unsafe fn load_data_raw(self,
                                target: GLenum, ptr: *const c_void,
                                size: GLsizeiptr, usage: GLenum) -> Self {
        self.bind(target);
        gl::BufferData(target, size, ptr, usage);

        self.unbind(target);

        self
    }

    pub fn load_data<T>(self, target: GLenum, data: &[T], usage: GLenum) -> Self {
        let ptr = data.as_ptr() as *const c_void;
        let size = mem::size_of::<T>() * data.len();

        unsafe {
            self.load_data_raw(target, ptr, size as GLsizeiptr, usage)
        }
    }

    pub fn load<T>(self, target: GLenum, buffer: &geom::mesh::Buffer<T>, usage: GLenum) -> Self {
        self.load_data(target, &buffer.buffer, usage)
    }

    pub fn bind(&self, target: GLenum) {
        unsafe {
            gl::BindBuffer(target, self.buffer);
        }
    }

    pub fn unbind(&self, target: GLenum) {
        unsafe {
            gl::BindBuffer(target, 0);
        }
    }

}

/// A collection of buffers, representing one set of data.
pub struct Data
{
    buffers: Vec<Buffer>,
}

impl Data
{
    pub fn new() -> Self {
        Data {
            buffers: Vec::new(),
        }
    }

    pub fn load<T>(mut self, target: GLenum, data: &geom::mesh::Data<T>, usage: GLenum) -> Self {
        self.buffers.extend(data.buffers()
                                .map(|b| Buffer::new().load(target, b, usage)));
        self
    }
}

/// A geometric mesh loaded into memory.
pub struct Mesh
{
    indices: Data,
    vertices: Data,
}

impl Mesh
{
    pub fn new() -> Self {
        Mesh {
            indices: Data::new(),
            vertices: Data::new(),
        }
    }

    pub fn load<I,V>(mut self, mesh: &geom::Mesh<I,V>) -> Mesh {
        self.indices = self.indices.load(gl::ELEMENT_ARRAY_BUFFER, &mesh.indices, gl::STATIC_DRAW);
        self.vertices = self.vertices.load(gl::ARRAY_BUFFER, &mesh.vertices, gl::STATIC_DRAW);
        self
    }

}
