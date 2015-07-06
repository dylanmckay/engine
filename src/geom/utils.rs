
use geom;
use std;

/// A face triangulator.
pub struct Triangulate<F: geom::Face> {
    face: F,
    cur_idx: u16,
    face_vertex_count: u16,
    is_already_triangular: bool,
    first_point: F::Vertex,
}

/// Triangulates a face.
///
/// If the face has more than three points,
/// it is coerced into a set of faces which are triangular.
pub fn triangulate<V,F>(face: F) -> Triangulate<F>
    where V: Copy + Clone,
          F: geom::Face<Vertex=V> + std::iter::FromIterator<V> {

    Triangulate {
        cur_idx: 0,
        face_vertex_count: face.vertices().count() as u16,
        is_already_triangular: face.is_triangular(),
        first_point: face.vertices().next().unwrap().clone(),
        face: face,
    }
}

impl<V,F> Iterator for Triangulate<F>
    where V: Copy + Clone,
          F: geom::Face<Vertex=V> + std::iter::FromIterator<V>
{
    type Item = F;

    fn next(&mut self) -> Option<F> {
        self.cur_idx += 1;

        if self.is_already_triangular {
            // return the face if we haven't already
            if self.cur_idx == 1 {
                Some(self.face.clone())
            } else { // we have already returned the face before
                None
            }
        } else { // we must triangulate the face

            if self.cur_idx >= self.face_vertex_count {
                return None;
            }

            let vertices = self.face.vertices().cloned();

            let pts = std::iter::once(self.first_point)
                                 .chain(vertices.skip(self.cur_idx as usize).take(2));

            let triangular_face = F::from_iter(pts);
            assert!(triangular_face.is_triangular());

            Some(triangular_face)
        }
    }
}
