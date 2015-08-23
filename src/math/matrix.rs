
use math;
use num::Primitive;

pub trait Matrix<T: Primitive> : Sized
{
    /// Gets the identity matrix.
    // TODO: Make this an associated constant.
    fn identity() -> Self {
        Self::from_fn(math::util::kronecker_delta)
    }

    /// Creates a matrix from a function taking row and column numbers.
    fn from_fn<F>(f: F) -> Self
        where F: Fn(u32,u32) -> T;

    /// Gets the element at the given index.
    /// Panics on out of range.
    fn element(&self, row: u32, col: u32) -> T;
}
