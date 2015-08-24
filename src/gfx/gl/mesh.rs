
use gfx::gl::gl;
use gfx::gl::vertex;
use gfx::gl::gl::types::*;
use gfx::gl::Type;
use gfx::gl::util;
use geom;
use libc::c_void;
use std::{self, mem};

/// A raw hardware buffer.
#[derive(Copy,Clone)]
pub struct RawBuffer
{
    /// The OpenGL buffer object,
    buffer: GLuint,
    /// The number of bytes in the buffer.
    /// This can be zero.
    size: GLsizeiptr,
}

impl RawBuffer
{
    fn new(buffer: GLuint, size: GLsizeiptr) -> Self {
        RawBuffer {
            buffer: buffer,
            size: size,
        }
    }

    fn empty() -> Self {
        let mut buffer = unsafe { mem::uninitialized() };

        unsafe {
            gl::GenBuffers(1, &mut buffer as *mut _);
        }

        RawBuffer::new(buffer, 0)
    }

    pub unsafe fn load_raw(mut self, target: GLenum, ptr: *const c_void,
                           size: GLsizeiptr, usage: GLenum) -> Self {

        self.bind(target);
        gl::BufferData(target, size, ptr, usage);
        self.unbind(target);

        self.size = size;

        self
    }

    /// Binds the buffer to the specified target.
    pub fn bind(&self, target: GLenum) {
        unsafe {
            gl::BindBuffer(target, self.buffer);
        }
    }

    /// Unbinds the buffer from the specified target.
    pub fn unbind(&self, target: GLenum) {
        unsafe {
            gl::BindBuffer(target, 0);
        }
    }
}

/// A mesh buffer.
pub struct Buffer
{
    index_buffer: RawBuffer,
    vertex_buffer: RawBuffer,

    /// The OpenGL index type enum.
    /// `gl::INVALID_ENUM` if it is not set.
    index_type: GLenum,
    pub formats: Vec<vertex::FormatInfo>,
}

impl Buffer
{
    pub fn from_raw(index_buffer: RawBuffer,
                    vertex_buffer: RawBuffer,
                    index_type: GLenum,
                    formats: Vec<vertex::FormatInfo>) -> Buffer {
        Buffer {
            index_buffer: index_buffer,
            vertex_buffer: vertex_buffer,
            index_type: index_type,
            formats: formats,
        }
    }

    pub fn empty() -> Self {
        Buffer {
            index_buffer: RawBuffer::empty(),
            vertex_buffer: RawBuffer::empty(),
            index_type: gl::INVALID_ENUM,
            formats: Vec::new(),
        }
    }

    pub fn load<I,V>(buffer: &geom::mesh::Buffer<I,V>, usage: GLenum)
        -> Self
        where I: Type, V: vertex::Vertex {
        let mut buf = Buffer::empty();

        buf.load_buffer(buffer, usage);
        buf
    }

    pub unsafe fn load_index_data_raw(&mut self,
                                      ptr: *const c_void,
                                      size: GLsizeiptr, usage: GLenum,
                                      ty: GLenum) {
        self.index_type = ty;

        self.bind_indices();
        self.index_buffer.load_raw(gl::ELEMENT_ARRAY_BUFFER, ptr, size, usage);
        self.unbind_indices();
    }

    pub unsafe fn load_vertex_data_raw(&mut self,
                                       ptr: *const c_void,
                                       size: GLsizeiptr, usage: GLenum)  {
        self.bind_vertices();
        self.vertex_buffer.load_raw(gl::ARRAY_BUFFER, ptr, size, usage);
        self.unbind_vertices();
    }

    /// Loads index data from a slice.
    pub fn load_index_data<T>(&mut self, data: &[T], usage: GLenum)
        where T: Type {

        let ptr = data.as_ptr() as *const c_void;
        let size = (data.len() * mem::size_of::<T>()) as GLsizeiptr;

        unsafe {
            self.load_index_data_raw(ptr, size, usage, T::gl_type())
        }
    }
    
    /// Loads vertex data from a slice.
    pub fn load_vertex_data<T>(&mut self, data: &[T], usage: GLenum)
        where T: vertex::Vertex {
        let ptr = data.as_ptr() as *const c_void;
        let size = (data.len() * mem::size_of::<T>()) as GLsizeiptr;

        self.formats = T::formats();

        unsafe {
            self.load_vertex_data_raw(ptr, size, usage)
        }
    }

    /// Loads a mesh buffer.
    pub fn load_buffer<I,V>(&mut self, buffer: &geom::mesh::Buffer<I,V>, usage: GLenum)
        where I: Type, V: vertex::Vertex {
        self.load_index_data(&buffer.indices, usage);
        self.load_vertex_data(&buffer.vertices, usage);
    }

    /// Gets the number of indices in the buffer.
    pub fn index_count(&self) -> usize {
        let index_total_size = self.index_buffer.size;
        let index_size = util::size_of_type(self.index_type()) as GLsizeiptr;

        // Assert that this obvious case is true.
        debug_assert!(index_total_size % index_size == 0,
                      "the indices must be tightly packed in the array");

        (index_total_size / index_size) as usize
    }

    /// Gets the type of indices in the buffer.
    /// Panics if there are no indices loaded.
    pub fn index_type(&self) -> GLenum {
        assert!(self.index_type != gl::INVALID_ENUM,
                "no indices are loaded");
        self.index_type
    }

    /// Gets the raw index buffer.
    pub fn raw_index_buffer<'a>(&'a self) -> &'a RawBuffer {
        &self.index_buffer
    }

    /// Gets the raw vertex buffer.
    pub fn raw_vertex_buffer<'a>(&'a self) -> &'a RawBuffer {
        &self.vertex_buffer
    }

    /// Binds the index array.
    pub fn bind_indices(&self) {
        self.index_buffer.bind(gl::ELEMENT_ARRAY_BUFFER);
    }

    /// Binds the vertex array.
    pub fn bind_vertices(&self) {
        self.vertex_buffer.bind(gl::ARRAY_BUFFER);
    }

    /// Unbinds the index array.
    pub fn unbind_indices(&self) {
        self.index_buffer.unbind(gl::ELEMENT_ARRAY_BUFFER);
    }

    /// Unbinds the vertex array.
    pub fn unbind_vertices(&self) {
        self.vertex_buffer.unbind(gl::ARRAY_BUFFER);
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

    pub fn load<I,V>(mut self, data: &geom::mesh::Data<I,V>, usage: GLenum) -> Self
        where I: Type, V: vertex::Vertex {
        self.buffers.extend(data.buffers()
                                .map(|b| Buffer::load(b, usage)));
        self
    }

    pub fn buffers<'a>(&'a self) -> std::slice::Iter<'a, Buffer> {
        self.buffers.iter()
    }
}

