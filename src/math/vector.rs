
use math::util;
use num::{Num,Decimal};
use std;

pub trait Vector<T: Num> : std::iter::FromIterator<T> + Copy + Clone {
    fn components<'a>(&'a self) -> util::Components<'a, T>;

    /// Calculates the squared length.
    fn length_squared(self) -> T;

    /// Calculates the length.
    fn length(self) -> T
        where T: Decimal {
        self.length_squared().sqrt()
    }

    /// Calculates the reciprocal of the length.
    fn length_inverse(self) -> T
        where T: Decimal {
        self.length_squared().rsqrt()
    }
}

