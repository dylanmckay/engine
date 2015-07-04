
use num;

pub trait Color : Copy + Clone { }

/// An RGBA value.
#[derive(Copy,Clone)]
pub struct RGBA<T: num::Integer = u8>(pub T, pub T, pub T, pub T);

impl<T: num::Integer> RGBA<T>
{
    pub fn map<U, F>(self, f: F) -> RGBA<U>
        where U: num::Integer, F: Fn(T) -> U {
        let RGBA(r,g,b,a) = self;

        let nr = f(r);
        let ng = f(g);
        let nb = f(b);
        let na = f(a);

        RGBA(nr,ng,nb,na)
    }
}

impl<T: num::Integer> Color for RGBA<T> { }

#[derive(Copy,Clone)]
pub struct NormalizedRGBA<T: num::Decimal = f32>(pub T, pub T, pub T, pub T);

impl<T: num::Decimal> NormalizedRGBA<T>
{
    pub fn map<U, F>(self, f: F) -> NormalizedRGBA<U>
        where U: num::Decimal, F: Fn(T) -> U {
        let NormalizedRGBA(r,g,b,a) = self;

        let nr = f(r);
        let ng = f(g);
        let nb = f(b);
        let na = f(a);

        NormalizedRGBA(nr,ng,nb,na)
    }
   
}

impl<T: num::Decimal> Color for NormalizedRGBA<T> { }

