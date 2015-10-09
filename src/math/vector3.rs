use std;
use std::ops;
use num::{self,Num};
use math::{util,Scalar};

#[repr(C)]
#[derive(Copy,Clone)]
pub struct Vector3<T: Num = Scalar>(pub T,pub T,pub T);

impl<T: Num> Vector3<T>
{
    /// Creates a vector with identical components.
    pub fn with_components(val: T) -> Self {
        Vector3(val, val, val)
    }

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

    pub fn dot(self, other: Self) -> T {
        let (x1,y1,z1) = self.into();
        let (x2,y2,z2) = other.into();

        x1*x2 + y1*y2 + z1*z2
    }

    pub fn cross(self, other: Self) -> Self {
        let (x1,y1,z1) = self.into();
        let (x2,y2,z2) = other.into();

        let x = y1 * z2 - z1 * y2;
        let y = z1 * x2 - x1 * z2;
        let z = x1 * y2 - y1 * x2;

        Vector3(x,y,z)
    }

    pub fn xyz(self) -> (T,T,T) { self.into() }
    pub fn xy(self) -> (T,T) {
        let Vector3(x,y,_) = self;
        (x,y)
    }

    pub fn x(self) -> T {
        let Vector3(x,_,_) = self;
        x
    }

    pub fn y(self) -> T {
        let Vector3(_,y,_) = self;
        y
    }

    pub fn z(self) -> T {
        let Vector3(_,_,z) = self;
        z
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
    pub fn as_positive(self) -> Self {
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

impl<T: Num> num::Zero for Vector3<T>
{
    fn zero() -> Self {
        Vector3(T::zero(), T::zero(), T::zero())
    }

    fn is_zero(self) -> bool {
        let (x,y,z) = self.into();
        x.is_zero() && y.is_zero() && z.is_zero()
    }
}

impl<T: Num> num::One for Vector3<T>
{
    fn one() -> Self {
        Vector3::with_components(T::one())
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

impl<T: Num> ops::Div for Vector3<T>
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let (x1,y1,z1) = self.into();
        let (x2,y2,z2) = rhs.into();

        Vector3(x1/x2, y1/y2, z1/z2)
    }
}

impl<T: Num> ops::Div<T> for Vector3<T>
{
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Vector3<T> {
        let Vector3(x,y,z) = self;
        Vector3(x/rhs, y/rhs, z/rhs)
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
