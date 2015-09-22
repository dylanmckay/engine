
use math::{self,Matrix};
use num::Num;
use std;

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct Matrix4<T: Num>
{
    m: [T; 16],
}

impl<T: Num> Matrix4<T>
{
    pub fn new(m11: T, m12: T, m13: T, m14: T,
               m21: T, m22: T, m23: T, m24: T,
               m31: T, m32: T, m33: T, m34: T,
               m41: T, m42: T, m43: T, m44: T) -> Self {
        Matrix4 {
            m: [
                m11, m12, m13, m14,
                m21, m22, m23, m24,
                m31, m32, m33, m34,
                m41, m42, m43, m44,
            ]
        }
    }
}

impl<T: Num> Matrix<T> for Matrix4<T>
{
    fn from_fn<F>(f: F) -> Self
        where F: Fn(usize,usize) -> T {
        Matrix4 {
            m: [
                f(0,0), f(0,1), f(0,2), f(0,3),
                f(1,0), f(1,1), f(1,2), f(1,3),
                f(2,0), f(2,1), f(2,2), f(2,3),
                f(3,0), f(3,1), f(3,2), f(3,3),
            ]
        }
    }

    fn get(&self, row: usize, col: usize) -> T {
        self.m[calculate_index(row as usize, col as usize)]
    }

    fn set(&mut self, row: usize, col: usize, val: T) {
        assert!(row < 4 && col < 4, "out of bounds indices");

        self.m[calculate_index(row as usize, col as usize)] = val;
    }

    fn row<'a>(&'a self, num: usize) -> math::matrix::row::Row<'a,T> {
        math::matrix::row::Row::new(&self.m, num, 4)
    }

    fn col<'a>(&'a self, num: usize) -> math::matrix::column::Column<'a,T> {
        math::matrix::column::Column::new(&self.m, num, 4, 4)
    }

    fn as_slice<'a>(&'a self)-> &'a [T] { &self.m }
    fn as_slice_mut<'a>(&'a mut self) -> &'a mut [T] { &mut self.m }
}

impl<T: Num> std::ops::Mul for Matrix4<T>
{
    type Output = Self;

    fn mul(self, rhs: Matrix4<T>) -> Matrix4<T> {
        let ((m11,m12,m13,m14),
             (m21,m22,m23,m24),
             (m31,m32,m33,m34),
             (m41,m42,m43,m44)) = self.into();
        let ((n11,n12,n13,n14),
             (n21,n22,n23,n24),
             (n31,n32,n33,n34),
             (n41,n42,n43,n44)) = rhs.into();

        let mn11 = m11*n11 + m12*n21 + m13*n31 + m14*n41;
        let mn12 = m11*n12 + m12*n22 + m13*n32 + m14*n42;
        let mn13 = m11*n13 + m12*n23 + m13*n33 + m14*n43;
        let mn14 = m11*n14 + m12*n24 + m13*n34 + m14*n44;

        let mn21 = m21*n11 + m22*n21 + m23*n31 + m24*n41;
        let mn22 = m21*n12 + m22*n22 + m23*n32 + m24*n42;
        let mn23 = m21*n13 + m22*n23 + m23*n33 + m24*n43;
        let mn24 = m21*n14 + m22*n24 + m23*n34 + m24*n44;

        let mn31 = m31*n11 + m32*n21 + m33*n31 + m34*n41;
        let mn32 = m31*n12 + m32*n22 + m33*n32 + m34*n42;
        let mn33 = m31*n13 + m32*n23 + m33*n33 + m34*n43;
        let mn34 = m31*n14 + m32*n24 + m33*n34 + m34*n44;

        let mn41 = m41*n11 + m42*n21 + m43*n31 + m44*n41;
        let mn42 = m41*n12 + m42*n22 + m43*n32 + m44*n42;
        let mn43 = m41*n13 + m42*n23 + m43*n33 + m44*n43;
        let mn44 = m41*n14 + m42*n24 + m43*n34 + m44*n44;

        Matrix4::new(mn11, mn12, mn13, mn14,
                     mn21, mn22, mn23, mn24,
                     mn31, mn32, mn33, mn34,
                     mn41, mn42, mn43, mn44)
    }
}

impl<T: Num> std::ops::Mul<Matrix4<T>> for math::Vector3<T>
{
    type Output = math::Vector3<T>;

    fn mul(self, m: Matrix4<T>) -> math::Vector3<T> {
        let (x,y,z) = self.into();
        let w = T::one();

        let mut xdash = x*m[(0,0)] + y*m[(1,0)] + z*m[(2,0)] + w*m[(3,0)];
        let mut ydash = x*m[(0,1)] + y*m[(1,1)] + z*m[(2,1)] + w*m[(3,1)];
        let mut zdash = x*m[(0,2)] + y*m[(1,2)] + z*m[(2,2)] + w*m[(3,2)];
        let wdash = x*m[(0,3)] + y*m[(1,3)] + z*m[(2,3)] + w*m[(3,3)];

        // Normalise homogenous coordinates
        if !wdash.is_zero() {
            xdash = xdash / wdash;
            ydash = ydash / wdash;
            zdash = zdash / wdash;
        }

        math::Vector3(xdash, ydash, zdash)
    }
}

impl<T: Num> std::ops::Index<(usize,usize)> for Matrix4<T>
{
    type Output = T;

    fn index<'a>(&'a self, (row,col): (usize,usize)) -> &'a T {
        &self.m[calculate_index(row,col)]
    }
}

impl<T: Num> std::ops::IndexMut<(usize,usize)> for Matrix4<T>
{
    fn index_mut<'a>(&'a mut self, (row,col): (usize,usize)) -> &'a mut T {
        &mut self.m[calculate_index(row,col)]
    }
}

impl<T: Num> Into<((T,T,T,T),(T,T,T,T),(T,T,T,T),(T,T,T,T))> for Matrix4<T>
{
    fn into(self) -> ((T,T,T,T),(T,T,T,T),(T,T,T,T),(T,T,T,T)) {
        ( (self[(0,0)], self[(0,1)], self[(0,2)], self[(0,3)] ),
          (self[(1,0)], self[(1,1)], self[(1,2)], self[(1,3)] ),
          (self[(2,0)], self[(2,1)], self[(2,2)], self[(2,3)] ),
          (self[(3,0)], self[(3,1)], self[(3,2)], self[(3,3)] ) )
    }
}

fn calculate_index(row: usize, col: usize) -> usize {
    row*4 + col
}

#[test]
fn test_mat4_mul() {
    let identity = Matrix4::identity();
    let mat1 = Matrix4::new(1.,2.,3.,4.,
                            1.,2.,3.,4.,
                            1.,2.,3.,4.,
                            1.,2.,3.,4.);
    let mat2 = Matrix4::new(9.,8.,7.,6.,
                            9.,8.,7.,6.,
                            9.,8.,7.,6.,
                            9.,8.,7.,6.);

    assert_eq!(mat1 * identity, mat1);
    assert_eq!(mat2 * identity, mat2);
    assert_eq!(mat2 * identity, identity * mat2);

    assert_eq!(mat1 * mat2, Matrix4::new(
        90., 80., 70., 60.,
        90., 80., 70., 60.,
        90., 80., 70., 60.,
        90., 80., 70., 60.,
    ));
}

