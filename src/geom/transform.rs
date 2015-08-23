
use math::{Scalar,Vector3,Matrix4x4};

/// A 3D transformation.
pub type Transform3<T=Scalar> = Matrix4x4<T>;

impl Transform3
{
    pub fn translate(self, offset: Vector3) -> Self {
        let cur_offset = self.get_translation();
        self.set_translation(cur_offset + offset)
    }

    pub fn get_translation(&self) -> Vector3 {
        let x = self[(0, 3)];
        let y = self[(1, 3)];
        let z = self[(2, 3)];


        Vector3(x,y,z)
    }

    pub fn set_translation(mut self, trans: Vector3) -> Self {
        let Vector3(x,y,z) = trans;

        self[(0,3)] = x;
        self[(1,3)] = y;
        self[(2,3)] = z;

        self
    }

    pub fn scale(self, factor: Vector3) -> Self {
        let cur_scale = self.get_scale();
        self.set_scale(cur_scale + factor)
    }

    pub fn get_scale(&self) -> Vector3 {
        Vector3(self[(0,0)], self[(1,1)], self[(2,2)])
    }

    pub fn set_scale(mut self, factor: Vector3) -> Self {
        let Vector3(x,y,z) = factor;
        self[(0,0)] = x;
        self[(1,1)] = y;
        self[(2,2)] = z;

        self
    }
}
