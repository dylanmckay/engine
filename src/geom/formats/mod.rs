
pub use self::wavefront::Wavefront;

use geom::mesh;
use std::io;

pub mod wavefront;

/// A generic geometry format.
pub trait Format<I, V>
{
    /// Loads the geometry into a mesh builder.
    fn load_with_builder<R>(read: R, builder: &mut mesh::Builder<I,V>)
        where R: io::Read;

    /// Loads the geometry and returns it as a mesh.
    fn load<R>(read: R) -> mesh::Data<I,V>
        where R: io::Read {
        let mut builder = mesh::Builder::new();

        Self::load_with_builder(read, &mut builder);

        builder.into()
    }
}
