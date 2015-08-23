
pub use self::uniform::Uniform;

use gfx::gl::gl;
use gfx::gl::gl::types::*;
use std::{fmt,ptr,mem,ffi};

#[derive(Copy,Clone)]
pub enum Kind
{
    Vertex,
    Fragment,
}

impl Kind
{
    pub fn into_gl(self) -> GLenum {
        match self {
            Kind::Vertex => gl::VERTEX_SHADER,
            Kind::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

impl fmt::Display for Kind
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match *self {
            Kind::Vertex => "vertex",
            Kind::Fragment => "fragment",
        }.fmt(fmt)
    }
}

pub struct Shader
{
    shader: GLuint,
}

impl Shader
{
    pub unsafe fn from_raw(shader: GLuint) -> Self {
        Shader {
            shader: shader,
        }
    }

    /// Compiles a shader.
    pub fn compile(kind: Kind, src: &str) -> Result<Self,String> {
        let src_buf = ffi::CString::new(src).unwrap();
        let mut src_buf_ptr = src_buf.as_ptr();
        
        unsafe {
            Shader::compile_raw(kind.into_gl(),
                                &mut src_buf_ptr as *const *const GLchar)
        }

    }

    /// Compiles a shader.
    /// `src` - A null terminated buffer containing the shader source.
    pub unsafe fn compile_raw(kind: GLenum, src: *const *const GLchar) -> Result<Self,String> {
        let shader: GLuint = gl::CreateShader(kind);

        gl::ShaderSource(shader, 1, src, ptr::null());

        gl::CompileShader(shader);

        let mut status: GLint = mem::uninitialized();
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status == gl::TRUE as GLint {
            Ok(Shader::from_raw(shader))
        } else { // an error occured while compiling
            let mut log_length: GLint = mem::uninitialized();
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);

            // make sure the log length is at least one byte
            assert!(log_length > 0);

            // allocate a buffer for the log (including null terminator)
            let mut log_buf: Vec<u8> = Vec::with_capacity(log_length as usize);
            log_buf.set_len(log_length as usize);

            // copy the compile log into the buffer
            gl::GetShaderInfoLog(shader, log_length,
                                 ptr::null_mut(), log_buf.as_mut_ptr() as *mut GLchar);

            let s = String::from_utf8(log_buf).unwrap();
            Err(s)
        }

    }
}

pub struct Program
{
    program: GLuint,
}

impl Program
{
    pub unsafe fn from_raw(program: GLuint) -> Self {
        Program {
            program: program,
        }
    }

    pub fn link<I>(shaders: I) -> Result<Self,String>
        where I: Iterator<Item=Shader> {

        let shader_handles: Vec<GLuint> = shaders.map(|a| a.shader).collect();

        unsafe {
            Program::link_raw(&shader_handles)
        }
    }

    pub unsafe fn link_raw(shaders: &[GLuint]) -> Result<Self,String> {

        let program: GLuint = gl::CreateProgram();

        for &shader in shaders.iter() {
            gl::AttachShader(program, shader);
        }

        gl::LinkProgram(program);

        for &shader in shaders.iter() {
            gl::DetachShader(program, shader);
        }

        let mut status = mem::uninitialized();
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        if status == gl::TRUE as GLint {
            Ok(Program::from_raw(program))
        } else { // failed to link
            let mut log_length: GLint = mem::uninitialized();
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);

            // make sure the log length is at least one byte
            assert!(log_length > 0);

            let mut log_buf: Vec<u8> = Vec::with_capacity(log_length as usize);
            log_buf.set_len(log_length as usize);

            gl::GetProgramInfoLog(program, log_length,
                                  ptr::null_mut(), log_buf.as_mut_ptr() as *mut GLchar);

            let s = String::from_utf8(log_buf).unwrap();
            Err(s)
        }
    }

    pub fn uniform<'a>(&'a self, name: &str) -> Uniform<'a> {
        let src_buf = ffi::CString::new(name).unwrap();

        let location = unsafe {
            gl::GetUniformLocation(self.program, src_buf.as_ptr())
        };

        unsafe {
            Uniform::from_location(self, location)
        }
    }

    pub fn enable(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

pub mod uniform
{
    use math;
    use gfx::gl::Program;
    use gfx::gl::gl::types::*;
    use gfx::gl::gl;

    /// An OpenGL shader uniform.
    pub struct Uniform<'a> {
        program: &'a Program,
        location: GLint,
    }

    impl<'a> Uniform<'a> {
        /// Creates a uniform from its location number.
        pub unsafe fn from_location(program: &'a Program,
                                    location: GLint) -> Self {
            Uniform {
                program: program,
                location: location,
            }
        }

        /// Sets the uniform with a value.
        // TODO: Handle errors
        pub fn set<T: Type>(&self, val: T) {
            self.program.enable();
            T::set(self.location, val);
            self.program.disable();
        }
    }

    /// A type that can be used in a uniform.
    // TODO: Implement for all values
    pub trait Type
    {
        fn set(loc: GLint, val: Self);
    }

    impl Type for f32 {
        fn set(loc: GLint, val: f32) {
            unsafe { gl::Uniform1f(loc, val) }
        }
    }

    impl Type for (f32,f32) {
        fn set(loc: GLint, (v1,v2): (f32,f32)) {
            unsafe { gl::Uniform2f(loc, v1, v2) }
        }
    }

    impl Type for (f32,f32,f32) {
        fn set(loc: GLint, (v1,v2,v3): (f32,f32,f32)) {
            unsafe { gl::Uniform3f(loc, v1, v2, v3) }
        }
    }

    impl Type for (f32,f32,f32,f32) {
        fn set(loc: GLint, (v1,v2,v3,v4): (f32,f32,f32,f32)) {
            unsafe { gl::Uniform4f(loc, v1, v2, v3, v4) }
        }
    }

    impl Type for math::Vector3<f32> {
        fn set(loc: GLint, math::Vector3(v1,v2,v3): math::Vector3<f32>) {
            unsafe { gl::Uniform3f(loc, v1,v2,v3) }
        }
    }

    impl Type for math::Matrix4<f32> {
        fn set(loc: GLint, mat: math::Matrix4<f32>) {
            use math::Matrix;
            let data = mat.as_slice();
            unsafe { gl::UniformMatrix4fv(loc, 1, gl::TRUE, data.as_ptr()) }
        }
    }
}
