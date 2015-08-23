use gfx::gl::gl;
use gfx::gl::gl::types::*;
use math;
use num;
use std::mem;

/// A vertex format.
pub trait Format
{
    fn info() -> FormatInfo;
}

/// Information about a vertex format.
#[derive(Copy,Clone,Debug)]
pub struct FormatInfo
{
    // the number of bytes in a component
    pub component_size: u16,
    pub component_count: u16,
    pub component_type: GLenum,
}

impl FormatInfo {
    pub fn empty() -> Self {
        FormatInfo {
            component_size: 0,
            component_count: 0,
            component_type: 0,
        }
    }

    pub fn total_size(self) -> usize {
        self.component_size as usize * self.component_count as usize
    }
}

/// A type which can be used by OpenGL.
pub trait Type : Sized
{
    fn gl_type() -> GLenum;
    fn size() -> usize;
}

/// An OpenGL vertex.
pub trait Vertex : Sized
{
    fn piece_formats() -> Vec<FormatInfo>;
    fn total_size() -> usize {
        Self::piece_formats().iter().fold(0, |a,f| a+f.total_size())
    }
}

macro_rules! impl_format {
    ($ty:ty) => {
        impl_format!($ty, 1; $ty);
        impl_format!($ty, 2; ($ty,$ty));
        impl_format!($ty, 3; ($ty,$ty,$ty));
        impl_format!($ty, 4; ($ty,$ty,$ty,$ty));
    };
    ($underlying:ty, $count:expr; $ty:ty)  => {
        impl Format for $ty {
            fn info() -> FormatInfo {
                FormatInfo {
                    component_size: <$underlying as Type>::size() as u16,
                    component_type: <$underlying as Type>::gl_type(),
                    component_count: $count,
                }
            }
        }
    }
}

impl_format!(u8);
impl_format!(i8);
impl_format!(u16);
impl_format!(i16);
impl_format!(u32);
impl_format!(i32);
impl_format!(f32);
impl_format!(f64);

impl<T: Type + num::Primitive> Format for math::Vector3<T>
{
    fn info() -> FormatInfo {
        FormatInfo {
            component_size: T::size() as u16,
            component_count: 3,
            component_type: T::gl_type(),
        }
    }
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
