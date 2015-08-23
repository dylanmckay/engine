
use gfx::gl::gl;
use gfx::gl::gl::types::*;
use std::mem;


/// A type which can be used by OpenGL.
pub trait Type : Sized
{
    fn gl_type() -> GLenum;
    fn size() -> usize;
}

macro_rules! impl_component {
    ($ty:ident, $val:ident) => {
        impl Type for $ty {
            fn gl_type() -> GLenum {
                gl::$val
            }

            fn size() -> usize {
                mem::size_of::<$ty>()
            }
        }
    }
}

impl_component!(u8, UNSIGNED_BYTE);
impl_component!(i8, BYTE);
impl_component!(u16, UNSIGNED_SHORT);
impl_component!(i16, SHORT);
impl_component!(u32, UNSIGNED_INT);
impl_component!(i32, INT);
impl_component!(f32, FLOAT);
impl_component!(f64, DOUBLE);
