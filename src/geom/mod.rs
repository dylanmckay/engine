
pub use self::formats::Format;
pub use self::transform::Transform3;

pub mod mesh;
pub mod transform;

pub mod formats;
pub mod utils;

use std;

pub trait Vertex<T> : Copy + Clone
{
    fn coords(self) -> (T,T,T);
}

pub trait Face: Clone
{
    type Vertex : Copy + Clone;

    fn vertices<'a>(&'a self) -> std::slice::Iter<'a, Self::Vertex>;

    fn is_triangular(&self) -> bool {
        self.vertices().count() == 3
    }
}

impl<I> Face for Vec<I> where I: Copy+Clone
{
    type Vertex = I;

    fn vertices<'a>(&'a self) -> std::slice::Iter<'a, Self::Vertex> {
        self.iter()
    }
}

