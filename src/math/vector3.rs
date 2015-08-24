use std;
use std::ops;
use num::{self,Num};
use math::{util,Scalar};

#[repr(C)]
#[derive(Copy,Clone)]
pub struct Vector3<T: Num = Scalar>(pub T,pub T,pub T);

impl<T: Num> Vector3<T>
{
    /// Maps from one vector to another.
    pub fn map<U, F>(self, f: F) -> Vector3<U>
        where U: Num, F: Fn(T) -> U {
        
        let Vector3(old_x,old_y,old_z) = self;

        let x = f(old_x);
        let y = f(old_y);
        let z = f(old_z);

        Vector3(x,y,z)
    }

    /// Folds the vector into a single value.
    pub fn fold<B, F>(self, init: B, mut f: F) -> B
        where F: FnMut(B, T) -> B {
        let mut val = init;

        let Vector3(x,y,z) = self;

        val = f(val, x);
        val = f(val, y);
        val = f(val, z);

        val
    }

    /// Casts the components to a different type.
    // TODO: Use higher-kinded types to move this into
    //       the Vector trait when possible.
    pub fn cast<V>(self) -> Vector3<V> where V: Num {
        self.map(|a| num::cast(a))
    }

    /// Calculates the squared length of the vector.
    pub fn length_squared(self) -> T {
        self.fold(T::zero(), |acc, comp| acc + comp*comp)
    }

    /// Takes the absolute value of all of the components.
    pub fn as_positive(self) -> Self
        where T: num::Signed {
        self.map(|c| c.abs())
    }

    pub fn components<'a>(&'a self) -> util::Components<'a, T> {
        let start: *const T = unsafe {std::mem::transmute(self) };

        util::Components::with_length(start, 3)
    }
}

impl<T: Num> std::iter::FromIterator<T> for Vector3<T>
{
    fn from_iter<I>(i: I) -> Self
        where I: IntoIterator<Item=T> {
        let mut it = i.into_iter();
        let x = it.next().unwrap();
        let y = it.next().unwrap();
        let z = it.next().unwrap();

        Vector3(x,y,z)
    }
}

impl<T: Num> Into<(T,T,T)> for Vector3<T>
{
    fn into(self) -> (T,T,T) {
        let Vector3(x,y,z) = self;
        (x,y,z)
    }
}

impl<T: Num> ops::Add for Vector3<T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        use std::iter::FromIterator;
        Vector3::from_iter(self.components().zip(rhs.components()).map(|(c1,c2)| c1+c2))
    }
}

impl<T: Num> ops::Sub for Vector3<T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        use std::iter::FromIterator;
        Vector3::from_iter(self.components().zip(rhs.components()).map(|(c1,c2)| c1-c2))
    }
}

impl<T: Num> ops::Neg for Vector3<T>
    where T: num::Signed
{
    type Output = Self;

    fn neg(self) -> Self {
        self.map(|c| -c)
    }
}

impl<T: Num> ops::Mul for Vector3<T>
{
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Vector3<T> {
        let Vector3(lhs_x,lhs_y,lhs_z) = self;
        let Vector3(rhs_x,rhs_y,rhs_z) = rhs;

        Vector3(lhs_x*rhs_x,
                lhs_y*rhs_y,
                lhs_z*rhs_z)
    }
}

impl<T: Num> ops::Mul<T> for Vector3<T>
{
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Vector3<T> {
        let Vector3(x,y,z) = self;
        Vector3(x*rhs, y*rhs, z*rhs)
    }
}

impl<T: Num> From<(T,T,T)> for Vector3<T> {
    fn from((x,y,z): (T,T,T)) -> Self {
        Vector3(x,y,z)
    }
}

#[test]
fn test_vec3_components_iter() {
    let vec = Vector3(1.0,2.0,3.0);
    let mut components = vec.components();

    assert_eq!(components.next().unwrap(), 1.0);
    assert_eq!(components.next().unwrap(), 2.0);
    assert_eq!(components.next().unwrap(), 3.0);
    assert_eq!(components.next(), None);
    assert_eq!(components.next(), None);
}
