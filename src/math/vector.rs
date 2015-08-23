
use math::util;
use std;

pub trait Vector<T: Copy> : std::iter::FromIterator<T> + Copy + Clone {
    fn components<'a>(&'a self) -> util::Components<'a, T>;
}

