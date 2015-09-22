
use math::{self,Matrix};
use num::Num;
use std;

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct Matrix3<T: Num>
{
    m: [T; 9],
}

impl<T: Num> Matrix3<T>
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

impl<T: Num> Matrix<T> for Matrix3<T>
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

impl<T: Num> std::ops::Mul for Matrix3<T>
{
    type Output = Self;

    fn mul(self, rhs: Matrix3<T>) -> Matrix3<T> {
        let ((m11,m12,m13),
             (m21,m22,m23),
             (m31,m32,m33)) = self.into();
        let ((n11,n12,n13),
             (n21,n22,n23),
             (n31,n32,n33)) = rhs.into();

        let mn11 = m11*n11 + m12*n21 + m13*n31;
        let mn12 = m11*n12 + m12*n22 + m13*n32;
        let mn13 = m11*n13 + m12*n23 + m13*n33;

        let mn21 = m21*n11 + m22*n21 + m23*n31;
        let mn22 = m21*n12 + m22*n22 + m23*n32;
        let mn23 = m21*n13 + m22*n23 + m23*n33;

        let mn31 = m31*n11 + m32*n21 + m33*n31;
        let mn32 = m31*n12 + m32*n22 + m33*n32;
        let mn33 = m31*n13 + m32*n23 + m33*n33;

        Matrix3::new(mn11, mn12, mn13,
                     mn21, mn22, mn23,
                     mn31, mn32, mn33)
    }
}

impl<T: Num> std::ops::Mul<Matrix3<T>> for math::Vector3<T>
{
    type Output = math::Vector3<T>;

    fn mul(self, m: Matrix3<T>) -> math::Vector3<T> {
        let (x,y,z) = self.into();

        let xdash = x*m[(0,0)] + y*m[(1,0)] + z*m[(2,0)];
        let ydash = x*m[(0,1)] + y*m[(1,1)] + z*m[(2,1)];
        let zdash = x*m[(0,2)] + y*m[(1,2)] + z*m[(2,2)];

        math::Vector3(xdash, ydash, zdash)
    }
}

impl<T: Num> std::ops::Index<(usize,usize)> for Matrix3<T>
{
    type Output = T;

    fn index<'a>(&'a self, (row,col): (usize,usize)) -> &'a T {
        &self.m[calculate_index(row,col)]
    }
}

impl<T: Num> std::ops::IndexMut<(usize,usize)> for Matrix3<T>
{
    fn index_mut<'a>(&'a mut self, (row,col): (usize,usize)) -> &'a mut T {
        &mut self.m[calculate_index(row,col)]
    }
}

impl<T: Num> Into<((T,T,T),(T,T,T),(T,T,T))> for Matrix3<T>
{
    fn into(self) -> ((T,T,T),(T,T,T),(T,T,T)) {
        ( (self[(0,0)], self[(0,1)], self[(0,2)] ),
          (self[(1,0)], self[(1,1)], self[(1,2)] ),
          (self[(2,0)], self[(2,1)], self[(2,2)] ) )
    }
}

fn calculate_index(row: usize, col: usize) -> usize {
    row*3 + col
}

#[test]
fn test_mat3_mul() {
    let identity = Matrix3::identity();
    let mat1 = Matrix3::new(1.,2.,3.,
                            4.,5.,6.,
                            7.,8.,9.);
    let mat2 = Matrix3::new(9.,8.,7.,
                            6.,5.,4.,
                            3.,2.,1.);

    assert_eq!(mat1 * identity, mat1);
    assert_eq!(mat2 * identity, mat2);
    assert_eq!(mat2 * identity, identity * mat2);

    assert_eq!(mat1 * mat2, Matrix3::new(
        30.,  24.,  18.,
        84.,  69.,  54.,
        138., 114., 90.
    ));
}
