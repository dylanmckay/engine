
use math::Matrix;
use num::Primitive;

pub struct Matrix4x4<T: Primitive>
{
    m: [T; 16],
}

impl<T: Primitive> Matrix4x4<T>
{
    pub fn new(m11: T, m12: T, m13: T, m14: T,
               m21: T, m22: T, m23: T, m24: T,
               m31: T, m32: T, m33: T, m34: T,
               m41: T, m42: T, m43: T, m44: T) -> Self {
        Matrix4x4 {
            m: [
                m11, m12, m13, m14,
                m21, m22, m23, m24,
                m31, m32, m33, m34,
                m41, m42, m43, m44,
            ]
        }
    }
}

impl<T: Primitive> Matrix<T> for Matrix4x4<T>
{
    fn from_fn<F>(f: F) -> Self
        where F: Fn(u32,u32) -> T {
        Matrix4x4 {
            m: [
                f(0,0), f(0,1), f(0,2), f(0,3),
                f(1,0), f(1,1), f(1,2), f(1,3),
                f(2,0), f(2,1), f(2,2), f(2,3),
                f(3,0), f(3,1), f(3,2), f(3,3),
            ]
        }
    }

    fn element(&self, row: u32, col: u32) -> T {
        self.m[calculate_index(row as usize, col as usize)]
    }
}

fn calculate_index(row: usize, col: usize) -> usize {
    row*4 + col
}
