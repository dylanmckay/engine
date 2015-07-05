
use gfx::gl::gl;
use gfx::gl::gl::types::*;
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

    pub unsafe fn load_raw(&self,
                           target: GLenum, ptr: *const c_void,
                           size: GLsizeiptr, usage: GLenum)  {
        self.bind(target);
        gl::BufferData(target, size, ptr, usage);

        self.unbind(target);
    }

    pub fn load<T>(&self, target: GLenum, data: &[T], usage: GLenum) {
        let ptr = data.as_ptr() as *const c_void;
        let size = mem::size_of::<T>() * data.len();

        unsafe {
            self.load_raw(target, ptr, size as GLsizeiptr, usage);
        }
    }
}

pub struct Mesh
{
    buffers: Vec<Buffer>,
}

impl Mesh
{
    pub fn new() -> Self {
        Mesh {
            buffers: Vec::new(),
        }
    }

}
