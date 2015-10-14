
use Vertex;
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
        self.points.into_iter()
    }
    
    pub fn into_points(self) -> std::vec::IntoIter<V> {
        let points: Vec<_> = self.points.iter().cloned().collect();
        points.into_iter()
    }
}

