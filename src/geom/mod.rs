
pub use self::formats::Format;
pub use self::triangle::Triangle;
pub use self::aabb::Aabb;
pub use self::octree::Octree;
pub use self::transform::Transform3;

pub mod mesh;
pub mod triangle;
pub mod aabb;
pub mod octree;
pub mod transform;

pub mod formats;
pub mod util;

use num;
use math;
use std;

pub trait Vertex : Copy + Clone
{
    type T: num::Num;

    fn coords(self) -> math::Vector3<Self::T>;
}

pub trait Face: Clone
{
    type Vertex : Copy + Clone;

    fn vertices<'a>(&'a self) -> std::slice::Iter<'a, Self::Vertex>;

    fn is_triangular(&self) -> bool {
        // Note that we must take into account that the iterator
        // has already processed one vertex.
        let c = self.vertices().count();
        c == 3
    }
}

impl<I> Face for Vec<I> where I: Copy+Clone
{
    type Vertex = I;

    fn vertices<'a>(&'a self) -> std::slice::Iter<'a, Self::Vertex> {
        self.iter()
    }
}

impl<T> Vertex for math::Vector3<T>
    where T: num::Num
{
    type T = T;

    fn coords(self) -> math::Vector3<T> { self }
}
