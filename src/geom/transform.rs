
use math::{Vector3,Matrix,Matrix3,Matrix4};
use num::Num;

/// A 3D transformation.
pub type Transform3<T> = Matrix4<T>;

impl<T: Num> Transform3<T>
{
    pub fn translate(self, offset: Vector3<T>) -> Self {
        let cur_offset = self.get_translation();
        self.set_translation(cur_offset + offset)
    }

    pub fn get_translation(&self) -> Vector3<T> {
        let x = self[(0, 3)];
        let y = self[(1, 3)];
        let z = self[(2, 3)];


        Vector3(x,y,z)
    }

    pub fn set_translation(mut self, trans: Vector3<T>) -> Self {
        let Vector3(x,y,z) = trans;

        self[(0,3)] = x;
        self[(1,3)] = y;
        self[(2,3)] = z;

        self
    }

    pub fn scale(self, factor: Vector3<T>) -> Self {
        let cur_scale = self.get_scale();
        self.set_scale(cur_scale + factor)
    }

    pub fn get_scale(&self) -> Vector3<T> {
        Vector3(self[(0,0)], self[(1,1)], self[(2,2)])
    }

    pub fn set_scale(mut self, factor: Vector3<T>) -> Self {
        let Vector3(x,y,z) = factor;
        self[(0,0)] = x;
        self[(1,1)] = y;
        self[(2,2)] = z;

        self
    }

    pub fn rotate(self, mat: Matrix3<T>) -> Self {
        let rot_mat = self.get_rotation() * mat;
        self.set_rotation(rot_mat)
    }

    pub fn set_rotation(mut self, mat: Matrix3<T>) -> Self {
        for row in 0..3 {
            for col in 0..3 {
                self[(row,col)] = mat[(row,col)];
            }
        }
        self
    }

    pub fn get_rotation(&self) -> Matrix3<T> {
        // Get the rotation submatrix
        Matrix3::from_fn(|i,j| self[(i,j)])
    }
}
