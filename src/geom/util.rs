
use geom;
use std;

/// A face triangulator.
pub struct Triangulate<F: geom::Face> {
    face: F,
    cur_idx: u16,

    // we cache a few variables to save recalculation.
    vert_count: u16,
    first_vert: F::Vertex,
}

/// Triangulates a face.
///
/// If the face has more than three points,
/// it is converted into a set of faces which are triangular.
pub fn triangulate<V,F>(face: F) -> Triangulate<F>
    where V: Copy + Clone,
          F: geom::Face<Vertex=V> + std::iter::FromIterator<V> {

    Triangulate {
        vert_count: face.vertices().count() as u16,
        first_vert: face.vertices().map(|&a|a).next().unwrap(),
        face: face,
        cur_idx: 0,
    }
}

impl<V,F> Iterator for Triangulate<F>
    where V: Copy + Clone,
          F: geom::Face<Vertex=V> + std::iter::FromIterator<V>
{
    type Item = F;

    fn next(&mut self) -> Option<F> {

        self.cur_idx += 1;
        if self.cur_idx >= self.vert_count-1 {
            return None;
        }

        let vertices = self.face.vertices().cloned();

        let pts = std::iter::once(self.first_vert)
                             .chain(vertices.skip(self.cur_idx as usize).take(2));

        let triangular_face = F::from_iter(pts);
        assert!(triangular_face.is_triangular());

        Some(triangular_face)
    }
}
