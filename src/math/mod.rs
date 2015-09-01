
pub mod vector;
pub mod vector2;
pub mod vector3;
pub mod matrix;
pub mod matrix3;
pub mod matrix4;
pub mod quaternion;

pub mod util;

pub use self::vector::Vector;
pub use self::vector2::Vector2;
pub use self::vector3::Vector3;
pub use self::matrix::Matrix;
pub use self::matrix3::Matrix3;
pub use self::matrix4::Matrix4;
pub use self::quaternion::Quaternion;

/// A scalar.
pub type Scalar = f32;

/// A 2D axis.
pub enum Axis2
{
    Horizontal,
    Vertical,
}
