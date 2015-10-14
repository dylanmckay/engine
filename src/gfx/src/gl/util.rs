
use gl::gl;
use gl::gl::types::*;
use std::mem;


/// A type which can be used by OpenGL.
pub trait Type : Sized
{
    fn specifier() -> GLenum;
    fn size() -> usize;
}

/// Implements the `Type` trait for a GL-supported type.
macro_rules! impl_types {
    { $($ty:ident: $val:ident),* } => {

        $(
            impl Type for $ty {
                fn specifier() -> GLenum {
                    gl::$val
                }

                fn size() -> usize {
                    mem::size_of::<$ty>()
                }
            }
        )*

        /// Gets the size of a type specified
        /// by a `GLenum` value.
        pub fn size_of_type(ty: GLenum) -> usize {
            match ty {
                $(
                gl::$val => mem::size_of::<$ty>(),
                )*
                _ => unreachable!(),
            }
        }
    };
}

impl_types! {
    u8:  UNSIGNED_BYTE,
    i8:  BYTE,
    u16: UNSIGNED_SHORT,
    i16: SHORT,
    u32: UNSIGNED_INT,
    i32: INT,
    f32: FLOAT,
    f64: DOUBLE
}

/// Gets the culling mode `GLenum`.
pub fn culling_mode(mode: ::CullingMode) -> GLenum
{
    match mode {
        ::CullingMode::Front => gl::FRONT,
        ::CullingMode::Back => gl::BACK,
    }
}
