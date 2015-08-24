
use math;
use num::Decimal;
use std;

/// A quaternion.
/// The quaternion is of the form `(x,y,z,w)`.
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct Quaternion<T: Decimal>(pub T, pub T, pub T, pub T);

impl<T: Decimal> Quaternion<T>
{
    pub fn from_euler_radians(euler: math::Vector3<T>) -> Self {
        let half_euler = euler * (T::one()/(T::one()+T::one()));
        let (ex,ey,ez) = half_euler.into();

        let c1 = ex.cos();
        let c2 = ey.cos();
        let c3 = ez.cos();
        let s1 = ex.sin();
        let s2 = ey.sin();
        let s3 = ey.sin();

        let x = s1*s2*c3 + c1*c2*s3;
        let y = s1*c2*c3 + c1*s2*s3;
        let z = c1*s2*c3 - s1*c2*s3;
        let w = c1*c2*c3 - s1*s2*s3;

        Quaternion(x,y,z,w)
    }

    pub fn fold<B,F>(self, init: B, mut f: F) -> B
        where F: FnMut(B, T) -> B {
        let Quaternion(x,y,z,w) = self;

        let mut val = init;
        val = f(val, x);
        val = f(val, y);
        val = f(val, z);
        val = f(val, w);

        val
    }

    pub fn map<B, F>(self, mut f: F) -> Quaternion<B>
        where B: Decimal, F: FnMut(T) -> B {
        let Quaternion(ox,oy,oz,ow) = self;

        let x = f(ox);
        let y = f(oy);
        let z = f(oz);
        let w = f(ow);

        Quaternion(x,y,z,w)
    }

    /// Gets the squared length.
    pub fn length_squared(self) -> T {
        self.fold(T::zero(), |a,v| a + v*v)
    }

    /// Gets the length.
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }

    /// Gets the reciprocal of the length.
    pub fn length_inverse(self) -> T {
        self.length_squared().rsqrt()
    }

    /// Normalizes the quaternion.
    pub fn normalize(self) -> Self {
        let inverse_len = self.length_inverse();
        self.map(|a| a*inverse_len)
    }
}

impl<T: Decimal> std::ops::Mul for Quaternion<T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let Quaternion(x1,y1,z1,w1) = self;
        let Quaternion(x2,y2,z2,w2) = rhs;

        let x = w1*x2 + x1*w2 + y1*z2 - z1*y2;
        let y = w1*y2 - x1*z2 + y1*w2 + z1*x2;
        let z = w1*z2 + x1*y2 - y1*x2 + z1*w2;
        let w = w1*w2 - x1*x2 - y1*y2 - z1*z2;

        Quaternion(x,y,z,w)
    }
}

impl<T: Decimal> Into<(T,T,T,T)> for Quaternion<T>
{
    fn into(self) -> (T,T,T,T) {
        let Quaternion(x,y,z,w) = self;
        (x,y,z,w)
    }
}

#[test]
fn test_quaternion_multiplication() {
    let q1 = Quaternion(1.,2.,3.,4.);
    let q2 = Quaternion(4.,3.,2.,1.);

    assert_eq!(q1*q1, Quaternion(8.,16.,24.,2.));
    assert_eq!(q1*q2, Quaternion(12.,24.,6.,-12.));
}
