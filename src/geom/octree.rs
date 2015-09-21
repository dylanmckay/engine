
use geom::{Vertex,Triangle,Aabb};

const DEFAULT_THRESHOLD: usize = 40;

pub enum Octree<V: Vertex>
{
    Node {
        aabb: Aabb<V::T>,
        subtrees: [Box<Octree<V>>; 8],
    },
    Leaf {
        aabb: Aabb<V::T>,
        triangles: Vec<Triangle<V>>,
    }
}

impl<V> Octree<V>
    where V: Vertex
{
    pub fn build<I>(triangles: I,
                    threshold: usize)
        where I: Iterator<Item=Triangle<V>> {

        unimplemented!();
        //let points = triangles.flat_map(|t| t.points()).map(|&a|a);
    }

    pub fn build_recursive<I>(triangles: I,
                              aabb: Aabb<V::T>,
                              depth: usize,
                              threshold: usize,
                              maximum_depth: usize)
        where I: Iterator<Item=Triangle<V>> {
        unimplemented!();
    }

    /// Checks if a triangle is contained by the
    /// AABB of the octree.
    pub fn contains(&self, triangle: Triangle<V>) -> bool {
        let aabb = self.aabb();
        // check if any of the points are contained.
        triangle.points().any(|a| aabb.contains(a.coords()))
    }

    /// Gets the AABB of the octree.
    pub fn aabb(&self) -> Aabb<V::T> {
        match self {
            &Octree::Node { aabb, .. } => aabb,
            &Octree::Leaf { aabb, .. } => aabb,
        }
    }
}
