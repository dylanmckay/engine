
use math::{self,Scalar,Vector3,Matrix,Matrix3,Matrix4};
use num::{self,Num,Decimal};

/// A 3D transformation.
pub type Transform3<T = Scalar> = Matrix4<T>;

/// A 3D rotation matrix.
pub type Rotation3<T> = Matrix3<T>;

impl<T: Num> Transform3<T>
{
    /// Creates a perspective transformation.
    /// `fov` - the field of view in radians.
    pub fn perspective(fov: T, near: T, far: T, aspect: T) -> Self
        where T: Decimal + num::Signed {

        let fov_scale = num::one::<T>() / fov.tan();

        let m11 = fov_scale / aspect;
        let m22 = fov_scale;

        let m33 = -far / (far-near);
        let m43 = -(far*near)/(far-near);

        Transform3::new(
            m11,         num::zero(), num::zero(), num::zero(),
            num::zero(), m22,         num::zero(), num::zero(),
            num::zero(), num::zero(), m33,        -num::one::<T>(),
            num::zero(), num::zero(), m43,         num::one()
        )
    }

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
        self.set_scale(cur_scale * factor)
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

    pub fn rotate<R>(self, rot: R) -> Self
        where R: Into<Rotation3<T>> {

        let rot_mat = self.get_rotation() * rot.into();
        self.set_rotation(rot_mat)
    }

    pub fn set_rotation<R>(mut self, rot: R) -> Self
        where R: Into<Rotation3<T>> {

        let mat: Matrix3<T> = rot.into();
        for row in 0..3 {
            for col in 0..3 {
                self[(row,col)] = mat[(row,col)];
            }
        }
        self
    }

    pub fn get_rotation(&self) -> Rotation3<T> {
        // Get the rotation submatrix
        Matrix3::from_fn(|i,j| self[(i,j)])
    }
}

impl<T: Decimal> Into<Rotation3<T>> for math::Quaternion<T>
{
    fn into(self) -> Rotation3<T> {
        self.as_rotation_matrix()
    }
}

impl<T: Decimal+num::Signed> Into<Rotation3<T>> for math::Vector3<T>
{
    fn into(self) -> Rotation3<T> {
        let (x,y,z) = self.into();
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

        Rotation3::new(m11, m12, m13,
                       m21, m22, m23,
                       m31, m32, m33)

    }
}
