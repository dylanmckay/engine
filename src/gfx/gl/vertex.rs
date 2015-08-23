use gfx::gl::gl;
use gfx::gl::gl::types::*;
use math;
use num;
use std::mem;

#[derive(Copy,Clone,Debug)]
pub struct Format
{
    // the number of bytes in a component
    pub component_size: u16,
    pub component_count: u16,
    pub component_type: GLenum,
}

impl Format {
    pub fn empty() -> Self {
        Format {
            component_size: 0,
            component_count: 0,
            component_type: 0,
        }
    }

    pub fn total_size(self) -> usize {
        self.component_size as usize * self.component_count as usize
    }
}

pub trait Type
{
    fn gl_type() -> GLenum;
}

pub trait Vertex : Sized
{
    fn piece_formats() -> Vec<Format>;
}

/// An OpenGL vertex
pub trait VertexPiece : Sized
{
    type Type : Type;

    fn component_count() -> u16 {
        let total_size = Self::total_size();
        let component_size = Self::component_size();

        // check that the size fits a whole number of components.
        // TODO: alignment could mess this up
        assert!(total_size % component_size == 0,
                "the vertex does not contain solely one component type");

        total_size / component_size
    }

    fn component_size() -> u16 {
        mem::size_of::<Self::Type>() as u16
    }

    fn total_size() -> u16 {
        mem::size_of::<Self>() as u16
    }

    fn format() -> Format {
        Format {
            component_size: Self::component_size(),
            component_count: Self::component_count(),
            component_type: Self::Type::gl_type(),
        }
    }
}

impl<T> VertexPiece for math::Vector3<T>
    where T: Type + num::Primitive {
    type Type = T;
}
//GL_BYTE, GL_UNSIGNED_BYTE, GL_SHORT, GL_UNSIGNED_SHORT, GL_INT, and GL_UNSIGNED_INT 

macro_rules! impl_component {
    ($ty:ident, $val:ident) => {
        impl Type for $ty {
            fn gl_type() -> GLenum {
                gl::$val
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
