
use std;
use std::ops::{Add,Sub,Mul,Div,Neg,Rem};

/// Something that can be casted.
pub trait Cast<T>
{
    fn cast(self) -> T;
}

/// A number which can be casted.
pub trait NumCast : Cast<i8>  + Cast<u8> +
                    Cast<i16> + Cast<u16> +
                    Cast<i32> + Cast<u32> +
                    Cast<i64> + Cast<u64> +
                    Cast<isize> + Cast<usize> +
                    Cast<f32> + Cast<f64> {
}

/// A number which has an additive identity.
pub trait Zero : Add<Output=Self>
{
    fn zero() -> Self;
    fn is_zero(self) -> bool;
}

/// A number which has a multipicative identity.
pub trait One: Mul<Output=Self>
{
    fn one() -> Self;
}

/// A number which is an integer.
pub trait Integer : Zero + One + Bounded + Copy + Clone +
                    Add<Output=Self> + Sub<Output=Self> +
                    Mul<Output=Self> + Div<Output=Self> +
                    Rem<Output=Self> + PartialEq + NumCast {}

/// A decimal number.
pub trait Decimal : Zero + One + Sized + Bounded + Copy + Clone +
                    Add<Output=Self> + Sub<Output=Self> +
                    Mul<Output=Self> + Div<Output=Self> +
                    Rem<Output=Self> + PartialEq + NumCast
{
    // constants
    fn pi() -> Self;
    fn tau() -> Self;
    fn two_pi() -> Self { Decimal::tau() }
    fn e() -> Self;
    
    // functions
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn recip(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn rsqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;
    fn exp(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
    
    fn root(self, n: Self) -> Self
    {
        self.powf(n.recip())
    }
}

/// A number which can be positive or negative.
pub trait Signed: Neg<Output=Self>
{
    fn abs(self) -> Self;
}

/// A number which does not have a sign.
pub trait Unsigned { }

/// A generalised number.
pub trait Num : Zero + One + Bounded +
                Add<Output=Self> + Sub<Output=Self> +
                Mul<Output=Self> + Div<Output=Self> +
                Rem<Output=Self> + PartialEq + NumCast
{

}

/// A primitive number.
pub trait Primitive: Copy + Clone + Num + NumCast + PartialOrd
{

}

/// A number which has an upper and lower bound.
pub trait Bounded
{
    fn min() -> Self;
    fn max() -> Self;
}

macro_rules! impl_cast {
    ($ty:ident, $to:ident) => {
        impl Cast<$to> for $ty {
            fn cast(self) -> $to {
                self as $to
            }
        }
    }
}

macro_rules! impl_numcast {
    ($ty:ident) => {
        impl NumCast for $ty { }

        impl_cast!($ty, i8);
        impl_cast!($ty, u8);
        impl_cast!($ty, i16);
        impl_cast!($ty, u16);
        impl_cast!($ty, i32);
        impl_cast!($ty, u32);
        impl_cast!($ty, i64);
        impl_cast!($ty, u64);
        impl_cast!($ty, usize);
        impl_cast!($ty, isize);
        impl_cast!($ty, f32);
        impl_cast!($ty, f64);
    }
}

/// Implements the `Zero` and `One` traits on a type.
macro_rules! impl_zero_one {
    ($ty:ident, $z: expr, $o: expr) => {
        impl Zero for $ty
        {
            fn zero() -> $ty { $z }
            fn is_zero(self) -> bool { self == $z }
        }
        
        impl One for $ty
        {
            fn one() -> $ty { $o }
        }
    }
}

macro_rules! impl_integer {
    ($ty:ident) => {
        impl Integer for $ty { }
    }
}

macro_rules! impl_decimal {
    ($ty:ident) => {
        impl Decimal for $ty
        {
            fn pi() -> $ty { ::std::$ty::consts::PI }
            fn tau() -> $ty { ::std::$ty::consts::PI * 2.0}
            fn e() -> $ty { ::std::$ty::consts::E }
            
            fn floor(self) -> $ty { $ty::floor(self) }
            fn ceil(self) -> $ty { $ty::ceil(self) }
            fn round(self) -> $ty { $ty::round(self) }
            fn trunc(self) -> $ty { $ty::trunc(self) }
            fn recip(self) -> $ty { $ty::recip(self) }
            fn powi(self, n: i32) -> $ty { $ty::powi(self, n) }
            fn powf(self, n: Self) -> $ty { $ty::powf(self, n) }
            fn sqrt(self) -> $ty { $ty::sqrt(self) }
            fn rsqrt(self) -> $ty { $ty::rsqrt(self) }
            fn cbrt(self) -> $ty { $ty::cbrt(self) }
            fn sin(self) -> $ty { $ty::sin(self) }
            fn cos(self) -> $ty { $ty::cos(self) }
            fn tan(self) -> $ty { $ty::tan(self) }
            fn asin(self) -> $ty { $ty::asin(self) }
            fn acos(self) -> $ty { $ty::acos(self) }
            fn atan(self) -> $ty { $ty::atan(self) }
            fn atan2(self, other: Self) -> $ty { $ty::atan2(self, other) }
            fn sinh(self) -> $ty { $ty::sinh(self) }
            fn cosh(self) -> $ty { $ty::cosh(self) }
            fn tanh(self) -> $ty { $ty::tanh(self) }
            fn asinh(self) -> $ty { $ty::asinh(self) }
            fn acosh(self) -> $ty { $ty::acosh(self) }
            fn atanh(self) -> $ty { $ty::atanh(self) }
            
            fn root(self, n: Self) -> $ty
            {
                self.powf(n.recip())
            }
            
            fn exp(self) -> $ty { $ty::exp(self) }
            fn log(self, base: Self) -> $ty { $ty::log(self, base) }
            
            fn to_degrees(self) -> $ty { $ty::to_degrees(self) }
            fn to_radians(self) -> $ty { $ty::to_radians(self) }
        }
    }
}

/// Implements the `Signed` trait on a type.
macro_rules! impl_signed {
    ($ty:ident, $t:ident) => {
        impl Signed for $ty
        {
            fn abs(self) -> $ty
            {
                $ty::abs(self)
            }
        }
    }
}

/// Implements the `Unsigned` trait on a type.
macro_rules! impl_unsigned {
    ($ty:ident) => {
        impl Unsigned for $ty { }
    }
}

/// Implements the `Num` and `Primitive` traits on a type.
macro_rules! impl_num_primitive {
    ($ty:ident) => {
    
        impl Num       for $ty { }
        impl Primitive for $ty { }
    }
}

/// Implements the `Bounded` trait on a type.
macro_rules! impl_bounded {
    ($ty:ident, $min:expr, $max:expr) => {
        impl Bounded for $ty
        {
            fn min() -> $ty { $min }
            fn max() -> $ty { $max }
        }
    }
}

impl_numcast!(u8);
impl_numcast!(u16);
impl_numcast!(u32);
impl_numcast!(u64);
impl_numcast!(usize);

impl_numcast!(i8);
impl_numcast!(i16);
impl_numcast!(i32);
impl_numcast!(i64);
impl_numcast!(isize);

impl_numcast!(f32);
impl_numcast!(f64);

// implement Zero + One for unsigned integral types
impl_zero_one!(u8,   0, 1);
impl_zero_one!(u16,  0, 1);
impl_zero_one!(u32,  0, 1);
impl_zero_one!(u64,  0, 1);
impl_zero_one!(usize, 0, 1);

// implement Zero + One for signed integral types
impl_zero_one!(i8,   0, 1);
impl_zero_one!(i16,  0, 1);
impl_zero_one!(i32,  0, 1);
impl_zero_one!(i64,  0, 1);
impl_zero_one!(isize,  0, 1);

// implement Zero + One for floating point types
impl_zero_one!(f32,  0.0, 1.0);
impl_zero_one!(f64,  0.0, 1.0);

// implement Integer for unsigned integer types
impl_integer!(u8);
impl_integer!(u16);
impl_integer!(u32);
impl_integer!(u64);
impl_integer!(usize);

// implement Integer for signed integer types
impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(isize);

// implement Decimal for floating point types
impl_decimal!(f32);
impl_decimal!(f64);

// implement Signed for signed integral types
impl_signed!(i8,  SignedInt);
impl_signed!(i16, SignedInt);
impl_signed!(i32, SignedInt);
impl_signed!(i64, SignedInt);
impl_signed!(isize, SignedInt);

// implement Signed for floating point types
impl_signed!(f32, Float);
impl_signed!(f64, Float);

// implement Unsigned for unsigned integral types
impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(usize);

// implement Num + Primitive for unsigned integral types
impl_num_primitive!(u8);
impl_num_primitive!(u16);
impl_num_primitive!(u32);
impl_num_primitive!(u64);
impl_num_primitive!(usize);

// implement Num + Primitive for signed integral types
impl_num_primitive!(i8);
impl_num_primitive!(i16);
impl_num_primitive!(i32);
impl_num_primitive!(i64);
impl_num_primitive!(isize);

// implement Num + Primitive for floating point types
impl_num_primitive!(f32);
impl_num_primitive!(f64);

// implement Bounded for unsigned integral types
impl_bounded!(u8,    std::u8::MIN,    std::u8::MAX);
impl_bounded!(u16,   std::u16::MIN,   std::u16::MAX);
impl_bounded!(u32,   std::u32::MIN,   std::u32::MAX);
impl_bounded!(u64,   std::u64::MIN,   std::u64::MAX);
impl_bounded!(usize, std::usize::MIN, std::usize::MAX);

// implement Bounded for signed integral types
impl_bounded!(i8,    std::i8::MIN,    std::i8::MAX);
impl_bounded!(i16,   std::i16::MIN,   std::i16::MAX);
impl_bounded!(i32,   std::i32::MIN,   std::i32::MAX);
impl_bounded!(i64,   std::i64::MIN,   std::i64::MAX);
impl_bounded!(isize, std::isize::MIN, std::isize::MAX);

// implement Bounded for floating point types
impl_bounded!(f32, std::f32::MIN, std::f32::MAX);
impl_bounded!(f64, std::f64::MIN, std::f64::MAX);

/// Casts a number from one type to another.
pub fn cast<T, U: Cast<T>>(n: U) -> T
{
    n.cast()
}

pub fn zero<T: Zero>() -> T
{
    Zero::zero()
}

pub fn one<T: One>() -> T
{
    One::one()
}

pub fn max<T: Bounded>() -> T
{
    Bounded::max()
}

pub fn min<T: Bounded>() -> T
{
    Bounded::min()
}


