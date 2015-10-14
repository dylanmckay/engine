
pub use self::wavefront::Wavefront;

use mesh;
use std::io;

pub mod wavefront;
pub mod tds;

/// A generic geometry format.
pub trait Format<I, V>
{
    /// Loads the geometry into a mesh builder.
    fn load_with_builder<R>(read: R, builder: &mut mesh::StaticBuilder<I,V>)
        where R: io::Read;

    /// Loads the geometry and returns it as a mesh.
    fn load<R>(read: R) -> mesh::StaticData<I,V>
        where R: io::Read {
        let mut builder = mesh::StaticBuilder::new();

        Self::load_with_builder(read, &mut builder);

        builder.into()
    }
}
