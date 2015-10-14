use std;
use std::ops;
use num::{self,Num};
use {util,Scalar};

#[repr(C)]
#[derive(Copy,Clone)]
pub struct Vector2<T: Num = Scalar>(pub T,pub T);

impl<T: Num> Vector2<T>
{
    /// Maps from one vector to another.
    pub fn map<U, F>(self, f: F) -> Vector2<U>
        where U: Num, F: Fn(T) -> U {
        
        let Vector2(old_x,old_y) = self;

        let x = f(old_x);
        let y = f(old_y);

        Vector2(x,y)
    }

    /// Folds the vector into a single value.
    pub fn fold<B, F>(self, init: B, mut f: F) -> B
        where F: FnMut(B, T) -> B {
        let mut val = init;

        let Vector2(x,y) = self;

        val = f(val, x);
        val = f(val, y);

        val
    }

    pub fn dot(self, other: Self) -> T {
        let (x1,y1) = self.into();
        let (x2,y2) = other.into();

        x1*x2 + y1*y2
    }

    /// Casts the components to a different type.
    // TODO: Use higher-kinded types to move this into
    //       the Vector trait when possible.
    pub fn cast<V>(self) -> Vector2<V> where V: Num {
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

        util::Components::with_length(start, 2)
    }
}

impl<T: Num> std::iter::FromIterator<T> for Vector2<T>
{
    fn from_iter<I>(i: I) -> Self
        where I: IntoIterator<Item=T> {
        let mut it = i.into_iter();
        let x = it.next().unwrap();
        let y = it.next().unwrap();

        Vector2(x,y)
    }
}

impl<T: Num> Into<(T,T)> for Vector2<T>
{
    fn into(self) -> (T,T) {
        let Vector2(x,y) = self;
        (x,y)
    }
}

impl<T: Num> ops::Add for Vector2<T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        use std::iter::FromIterator;
        Vector2::from_iter(self.components().zip(rhs.components()).map(|(c1,c2)| c1+c2))
    }
}

impl<T: Num> ops::Sub for Vector2<T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        use std::iter::FromIterator;
        Vector2::from_iter(self.components().zip(rhs.components()).map(|(c1,c2)| c1-c2))
    }
}

impl<T: Num> ops::Neg for Vector2<T>
    where T: num::Signed
{
    type Output = Self;

    fn neg(self) -> Self {
        self.map(|c| -c)
    }
}

impl<T: Num> ops::Mul for Vector2<T>
{
    type Output = Vector2<T>;

    fn mul(self, rhs: Vector2<T>) -> Vector2<T> {
        let Vector2(lhs_x,lhs_y) = self;
        let Vector2(rhs_x,rhs_y) = rhs;

        Vector2(lhs_x*rhs_x,
                lhs_y*rhs_y)
    }
}

impl<T: Num> ops::Mul<T> for Vector2<T>
{
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Vector2<T> {
        let Vector2(x,y) = self;
        Vector2(x*rhs, y*rhs)
    }
}

impl<T: Num> From<(T,T)> for Vector2<T> {
    fn from((x,y): (T,T)) -> Self {
        Vector2(x,y)
    }
}

#[test]
fn test_vec3_components_iter() {
    let vec = Vector2(1.0,2.0);
    let mut components = vec.components();

    assert_eq!(components.next().unwrap(), 1.0);
    assert_eq!(components.next().unwrap(), 2.0);
    assert_eq!(components.next(), None);
    assert_eq!(components.next(), None);
}
