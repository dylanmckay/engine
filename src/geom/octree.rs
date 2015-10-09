
use geom::{Vertex,Triangle,Aabb};

const DEFAULT_THRESHOLD: usize = 40;
const MAXIMUM_DEPTH: usize = 12;

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
    pub fn build<I>(triangles: I) -> Self
        where I: Iterator<Item=Triangle<V>> + Clone {

        let aabb = Aabb::containing(triangles.clone().flat_map(|tri| {
            tri.into_points().map(|p| p.coords())
        }));

        Octree::build_advanced(triangles, aabb, MAXIMUM_DEPTH, DEFAULT_THRESHOLD)
    }

    pub fn build_advanced<I>(triangles: I,
                             aabb: Aabb<V::T>,
                             depth: usize,
                             threshold: usize) -> Self
        where I: Iterator<Item=Triangle<V>> {

        let triangle_buf: Vec<Triangle<V>> = triangles.collect();


        // check if the tirnalge count is less than the threshold
        // or that we have reached the maximum depth
        if triangle_buf.len() < threshold || depth == 0 {
            Octree::Leaf {
                aabb: aabb,
                triangles: triangle_buf,
            }
        } else { // we should subdivide the octree
            
            let mut sub_octree_it = aabb.subdivide().into_iter().map(|sub_aabb| {
                let contained_tris = triangle_buf.iter().filter(|tri| {
                    aabb.contains_any(tri.points().map(|p|p.coords()))
                }).cloned();

                Box::new(
                    Octree::build_advanced(contained_tris, sub_aabb, depth-1, threshold)
                )
            });

            let sub_octrees = [
                sub_octree_it.next().unwrap(),
                sub_octree_it.next().unwrap(),
                sub_octree_it.next().unwrap(),
                sub_octree_it.next().unwrap(),
                sub_octree_it.next().unwrap(),
                sub_octree_it.next().unwrap(),
                sub_octree_it.next().unwrap(),
                sub_octree_it.next().unwrap(),
            ];

            Octree::Node {
                aabb: aabb,
                subtrees: sub_octrees,
            }
        }
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
