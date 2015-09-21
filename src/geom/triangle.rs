
use geom::Vertex;
use std;

#[derive(Copy,Clone,Debug)]
pub struct Triangle<V: Vertex>
{
    points: [V; 3],
}

impl<V> Triangle<V>
    where V: Vertex
{
    pub fn new(p1: V, p2: V, p3: V) -> Self {
        Triangle {
            points: [p1, p2, p3],
        }
    }

    pub fn points<'a>(&'a self) -> std::slice::Iter<'a,V> {
        self.points.iter()
    }
}

