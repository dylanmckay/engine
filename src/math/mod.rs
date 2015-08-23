
pub mod vector;
pub mod vector3;
pub mod matrix;
pub mod matrix4;
pub mod util;

pub use self::vector::Vector;
pub use self::vector3::Vector3;
pub use self::matrix::Matrix;
pub use self::matrix4::Matrix4;

/// A scalar.
pub type Scalar = f32;
