
use gfx::gl::gl;
use gfx::gl::gl::types::*;
use geom;
use libc::c_void;
use std::{self, mem};

pub struct Buffer
{
    index_buffer: GLuint,
    pub index_count: usize,
    vertex_buffer: GLuint,
    pub vertex_count: usize,
}

impl Buffer
{
    pub unsafe fn from_raw(index_buffer: GLuint, index_count: usize,
                           vertex_buffer: GLuint, vertex_count: usize) -> Buffer {
        Buffer {
            index_buffer: index_buffer,
            index_count: index_count,
            vertex_buffer: vertex_buffer,
            vertex_count: vertex_count,
        }
    }

    pub fn new() -> Self {
        let mut buffers: [GLuint; 2] = unsafe {mem::uninitialized() };

        unsafe {
            gl::GenBuffers(2, buffers.as_mut_ptr());
            Buffer::from_raw(buffers[0], 0, buffers[1], 0)
        }
    }

    pub unsafe fn load_index_data_raw(self,
                                      ptr: *const c_void,
                                      size: GLsizeiptr, usage: GLenum) -> Self {
        self.bind_indices();
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, ptr, usage);
        self.unbind_indices();

        self
    }

    pub unsafe fn load_vertex_data_raw(self,
                                       ptr: *const c_void,
                                       size: GLsizeiptr, usage: GLenum) -> Self {
        self.bind_vertices();
        gl::BufferData(gl::ARRAY_BUFFER, size, ptr, usage);
        self.unbind_vertices();

        self
    }

    pub fn load_index_data<T>(mut self, data: &[T], usage: GLenum) -> Self {
        let ptr = data.as_ptr() as *const c_void;
        let size = mem::size_of::<T>() * data.len();

        self.index_count = data.len();

        unsafe {
            self.load_index_data_raw(ptr, size as GLsizeiptr, usage)
        }
    }
    
    pub fn load_vertex_data<T>(mut self, data: &[T], usage: GLenum) -> Self {
        let ptr = data.as_ptr() as *const c_void;
        let size = mem::size_of::<T>() * data.len();

        self.vertex_count = data.len();

        unsafe {
            self.load_vertex_data_raw(ptr, size as GLsizeiptr, usage)
        }
    }

    pub fn load<I,V>(self, buffer: &geom::mesh::Buffer<I,V>, usage: GLenum) -> Self {
        self.load_index_data(&buffer.indices, usage)
            .load_vertex_data(&buffer.vertices, usage)
    }

    pub fn bind_indices(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
        }
    }

    pub fn bind_vertices(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
        }
    }

    pub fn unbind_indices(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
        }
    }

    pub fn unbind_vertices(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
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

    pub fn load<I,V>(mut self, data: &geom::mesh::Data<I,V>, usage: GLenum) -> Self {
        self.buffers.extend(data.buffers()
                                .map(|b| Buffer::new().load(b, usage)));
        self
    }

    pub fn buffers<'a>(&'a self) -> std::slice::Iter<'a, Buffer> {
        self.buffers.iter()
    }
}

