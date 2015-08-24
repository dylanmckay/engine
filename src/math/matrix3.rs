
use math::{self,Matrix};
use num::Primitive;
use std;

pub struct Matrix3<T: Primitive>
{
    m: [T; 9],
}

impl<T: Primitive> Matrix3<T>
{
    pub fn new(m11: T, m12: T, m13: T,
               m21: T, m22: T, m23: T,
               m31: T, m32: T, m33: T) -> Self {
        Matrix3 {
            m: [
                m11, m12, m13,
                m21, m22, m23,
                m31, m32, m33,
            ]
        }
    }
}

impl<T: Primitive> Matrix<T> for Matrix3<T>
{
    fn from_fn<F>(f: F) -> Self
        where F: Fn(usize,usize) -> T {
        Matrix3 {
            m: [
                f(0,0), f(0,1), f(0,2),
                f(1,0), f(1,1), f(1,2),
                f(2,0), f(2,1), f(2,2),
            ]
        }
    }

    fn get(&self, row: usize, col: usize) -> T {
        self.m[calculate_index(row as usize, col as usize)]
    }

    fn set(&mut self, row: usize, col: usize, val: T) {
        assert!(row < 3 && col < 3, "out of bounds indices");

        self.m[calculate_index(row as usize, col as usize)] = val;
    }

    fn row<'a>(&'a self, num: usize) -> math::matrix::row::Row<'a,T> {
        math::matrix::row::Row::new(&self.m, num, 3)
    }

    fn col<'a>(&'a self, num: usize) -> math::matrix::column::Column<'a,T> {
        math::matrix::column::Column::new(&self.m, num, 3, 3)
    }

    fn as_slice<'a>(&'a self)-> &'a [T] { &self.m }
    fn as_slice_mut<'a>(&'a mut self) -> &'a mut [T] { &mut self.m }
}

impl<T: Primitive> std::ops::Index<(usize,usize)> for Matrix3<T>
{
    type Output = T;

    fn index<'a>(&'a self, (row,col): (usize,usize)) -> &'a T {
        &self.m[calculate_index(row,col)]
    }
}

impl<T: Primitive> std::ops::IndexMut<(usize,usize)> for Matrix3<T>
{
    fn index_mut<'a>(&'a mut self, (row,col): (usize,usize)) -> &'a mut T {
        &mut self.m[calculate_index(row,col)]
    }
}

fn calculate_index(row: usize, col: usize) -> usize {
    row*3 + col
}
