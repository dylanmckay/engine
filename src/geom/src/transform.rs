
use math::{self,Scalar,Vector3,Matrix,Matrix3,Matrix4};
use num::{self,Num,Decimal};
use std;

/// A 3D transformation.
#[derive(Copy,Clone,Debug)]
pub struct Transform3<T: Num = Scalar>
{
    matrix: Matrix4<T>,
}

/// A 3D rotation matrix.
#[derive(Copy,Clone,Debug)]
pub struct Rotation3<T: Num+Decimal>
{
    matrix: Matrix3<T>,
}

impl<T: Num+Decimal> Rotation3<T>
{
    pub fn from_matrix(mat: Matrix3<T>) -> Self {
        Rotation3 {
            matrix: mat,
        }
    }

    pub fn identity() -> Self {
        Rotation3::from_matrix(Matrix::identity())
    }

    pub fn from_quaternion(quat: math::Quaternion<T>) -> Self {
        Rotation3 {
            matrix: quat.as_rotation_matrix(),
        }
    }

    pub fn from_vector(vec: Vector3<T>) -> Self
        where T: num::Signed {

        let (x,y,z) = vec.into();
        // yezdiny => y
        // zttitude => z
        // xznk => x
        let (sx,cx) = x.sincos();
        let (sy,cy) = y.sincos();
        let (sz,cz) = z.sincos();

        let m11 = cy*cz;
        let m12 = -cy*sz*cx + sy*sx;
        let m13 = cy*sz*sx + sy*cx;
        let m21 = sz;
        let m22 = cz*cx;
        let m23 = -cz*sx;
        let m31 = -sy*cz;
        let m32 = sy*sz*cx + cy*sx;
        let m33 = -sy*sz*sx + cy*cx;

        Rotation3::from_matrix(
            Matrix3::new(m11, m12, m13,
                         m21, m22, m23,
                         m31, m32, m33)
        )

    }
}

impl<T: Num+Decimal> std::ops::Mul for Rotation3<T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Rotation3::from_matrix(self.matrix * rhs.matrix)
    }
}

impl<T: Num+Decimal> Transform3<T>
{
    pub fn from_matrix(mat: Matrix4<T>) -> Self {
        Transform3 {
            matrix: mat,
        }
    }

    pub fn identity() -> Self {
        Transform3::from_matrix(Matrix::identity())
    }

    /// Creates a perspective transformation.
    /// `fov` - the field of view in radians.
    pub fn perspective(fov: T, near: T, far: T, aspect: T) -> Self
        where T: Decimal + num::Signed {

        let fov_scale = num::one::<T>() / fov.tan();

        let m11 = fov_scale / aspect;
        let m22 = fov_scale;

        let m33 = -far / (far-near);
        let m43 = -(far*near)/(far-near);

        Transform3::from_matrix(
            Matrix4::new(
                m11,         num::zero(), num::zero(), num::zero(),
                num::zero(), m22,         num::zero(), num::zero(),
                num::zero(), num::zero(), m33,        -num::one::<T>(),
                num::zero(), num::zero(), m43,         num::one()
            )
        )
    }

    pub fn translate(self, offset: Vector3<T>) -> Self {
        let cur_offset = self.get_translation();
        self.set_translation(cur_offset + offset)
    }

    pub fn get_translation(&self) -> Vector3<T> {
        let x = self.matrix[(0, 3)];
        let y = self.matrix[(1, 3)];
        let z = self.matrix[(2, 3)];


        Vector3(x,y,z)
    }

    pub fn set_translation(mut self, trans: Vector3<T>) -> Self {
        let Vector3(x,y,z) = trans;

        self.matrix[(0,3)] = x;
        self.matrix[(1,3)] = y;
        self.matrix[(2,3)] = z;

        self
    }

    pub fn scale(self, factor: Vector3<T>) -> Self {
        let cur_scale = self.get_scale();
        self.set_scale(cur_scale * factor)
    }

    pub fn get_scale(&self) -> Vector3<T> {
        Vector3(self.matrix[(0,0)], self.matrix[(1,1)], self.matrix[(2,2)])
    }

    pub fn set_scale(mut self, factor: Vector3<T>) -> Self {
        let Vector3(x,y,z) = factor;
        self.matrix[(0,0)] = x;
        self.matrix[(1,1)] = y;
        self.matrix[(2,2)] = z;

        self
    }

    pub fn rotate<R>(self, rot: R) -> Self
        where R: Into<Rotation3<T>> {

        let rot_mat = self.get_rotation() * rot.into();
        self.set_rotation(rot_mat)
    }

    pub fn set_rotation<R>(mut self, rot: R) -> Self
        where R: Into<Rotation3<T>> {

        let rot3: Rotation3<T> = rot.into();
        let mat = rot3.matrix;

        for row in 0..3 {
            for col in 0..3 {
                self.matrix[(row,col)] = mat[(row,col)];
            }
        }
        self
    }

    pub fn get_rotation(&self) -> Rotation3<T> {

        // Get the rotation submatrix
        Rotation3::from_matrix(
            Matrix3::from_fn(|i,j| self.matrix[(i,j)])
        )
    }
}

impl<T: Num+Decimal+num::Signed> From<Vector3<T>> for Rotation3<T>
{
    fn from(vec: Vector3<T>) -> Rotation3<T> {
        Rotation3::from_vector(vec)
    }
}

impl<T: Num> Into<Matrix4<T>> for Transform3<T>
{
    fn into(self) -> Matrix4<T> {
        self.matrix
    }
}

impl<T: Num+Decimal> std::ops::Mul for Transform3<T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Transform3::from_matrix(self.matrix * rhs.matrix)
    }
}


