
#![feature(int_uint)]

use std::ops::{Add,Sub,Mul,Div,Neg,Rem};

pub trait Cast<T>
{
    fn cast(self) -> T;
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
pub trait Integer : Zero + One { }

/// A decimal number.
pub trait Decimal : Zero + One + Sized
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
pub trait Signed
{
    fn abs(self) -> Self;
}

/// A number which does not have a sign.
pub trait Unsigned { }

/// A generalised number.
pub trait Num : Zero + One +
                Add<Output=Self> + Sub<Output=Self> +
                Mul<Output=Self> + Div<Output=Self> +
                Rem<Output=Self> + Neg<Output=Self> +
                PartialEq
{

}

/// A primitive number.
pub trait Primitive: Copy + Clone + Num + std::num::NumCast + PartialOrd
{

}

/// A number which has an upper and lower bound.
pub trait Bounded
{
    fn min_value() -> Self;
    fn max_value() -> Self;
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
            fn tau() -> $ty { ::std::$ty::consts::PI_2 }
            fn e() -> $ty { ::std::$ty::consts::E }
            
            fn floor(self) -> $ty { ::std::num::Float::floor(self) }
            fn ceil(self) -> $ty { ::std::num::Float::ceil(self) }
            fn round(self) -> $ty { ::std::num::Float::round(self) }
            fn trunc(self) -> $ty { ::std::num::Float::trunc(self) }
            fn recip(self) -> $ty { ::std::num::Float::recip(self) }
            fn powi(self, n: i32) -> $ty { ::std::num::Float::powi(self, n) }
            fn powf(self, n: Self) -> $ty { ::std::num::Float::powf(self, n) }
            fn sqrt(self) -> $ty { ::std::num::Float::sqrt(self) }
            fn rsqrt(self) -> $ty { ::std::num::Float::rsqrt(self) }
            fn cbrt(self) -> $ty { ::std::num::Float::cbrt(self) }
            fn sin(self) -> $ty { ::std::num::Float::sin(self) }
            fn cos(self) -> $ty { ::std::num::Float::cos(self) }
            fn tan(self) -> $ty { ::std::num::Float::tan(self) }
            fn asin(self) -> $ty { ::std::num::Float::asin(self) }
            fn acos(self) -> $ty { ::std::num::Float::acos(self) }
            fn atan(self) -> $ty { ::std::num::Float::atan(self) }
            fn atan2(self, other: Self) -> $ty { ::std::num::Float::atan2(self, other) }
            fn sinh(self) -> $ty { ::std::num::Float::sinh(self) }
            fn cosh(self) -> $ty { ::std::num::Float::cosh(self) }
            fn tanh(self) -> $ty { ::std::num::Float::tanh(self) }
            fn asinh(self) -> $ty { ::std::num::Float::asinh(self) }
            fn acosh(self) -> $ty { ::std::num::Float::acosh(self) }
            fn atanh(self) -> $ty { ::std::num::Float::atanh(self) }
            
            fn root(self, n: Self) -> $ty
            {
                self.powf(n.recip())
            }
            
            fn exp(self) -> $ty { ::std::num::Float::exp(self) }
            fn log(self, base: Self) -> $ty { ::std::num::Float::log(self, base) }
            
            fn to_degrees(self) -> $ty { ::std::num::Float::to_degrees(self) }
            fn to_radians(self) -> $ty { ::std::num::Float::to_radians(self) }
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
                use std;
                
                std::num::$t::abs(self)
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

// implement Zero + One for unsigned integral types
impl_zero_one!(u8,   0, 1);
impl_zero_one!(u16,  0, 1);
impl_zero_one!(u32,  0, 1);
impl_zero_one!(u64,  0, 1);
impl_zero_one!(uint, 0, 1);

// implement Zero + One for signed integral types
impl_zero_one!(i8,   0, 1);
impl_zero_one!(i16,  0, 1);
impl_zero_one!(i32,  0, 1);
impl_zero_one!(i64,  0, 1);
impl_zero_one!(int,  0, 1);

// implement Zero + One for floating point types
impl_zero_one!(f32,  0.0, 1.0);
impl_zero_one!(f64,  0.0, 1.0);

// implement Integer for unsigned integer types
impl_integer!(u8);
impl_integer!(u16);
impl_integer!(u32);
impl_integer!(u64);
impl_integer!(uint);

// implement Integer for signed integer types
impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(int);

// implement Decimal for floating point types
impl_decimal!(f32);
impl_decimal!(f64);

// implement Signed for signed integral types
impl_signed!(i8,  SignedInt);
impl_signed!(i16, SignedInt);
impl_signed!(i32, SignedInt);
impl_signed!(i64, SignedInt);
impl_signed!(int, SignedInt);

// implement Signed for floating point types
impl_signed!(f32, Float);
impl_signed!(f64, Float);

// implement Unsigned for unsigned integral types
impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(uint);

// implement Num + Primitive for unsigned integral types
impl_num_primitive!(u8);
impl_num_primitive!(u16);
impl_num_primitive!(u32);
impl_num_primitive!(u64);
impl_num_primitive!(uint);

// implement Num + Primitive for signed integral types
impl_num_primitive!(i8);
impl_num_primitive!(i16);
impl_num_primitive!(i32);
impl_num_primitive!(i64);
impl_num_primitive!(int);

// implement Num + Primitive for floating point types
impl_num_primitive!(f32);
impl_num_primitive!(f64);

// implement Bounded for unsigned integral types
impl_bounded!(u8,   std::u8::MIN,   std::u8::MAX);
impl_bounded!(u16,  std::u16::MIN,  std::u16::MAX);
impl_bounded!(u32,  std::u32::MIN,  std::u32::MAX);
impl_bounded!(u64,  std::u64::MIN,  std::u64::MAX);
impl_bounded!(uint, std::uint::MIN, std::uint::MAX);

// implement Bounded for signed integral types
impl_bounded!(i8,   std::i8::MIN,   std::i8::MAX);
impl_bounded!(i16,  std::i16::MIN,  std::i16::MAX);
impl_bounded!(i32,  std::i32::MIN,  std::i32::MAX);
impl_bounded!(i64,  std::i64::MIN,  std::i64::MAX);
impl_bounded!(int,  std::int::MIN,  std::int::MAX);

// implement Bounded for floating point types
impl_bounded!(f32, std::f32::MIN, std::f32::MAX);
impl_bounded!(f64, std::f64::MIN, std::f64::MAX);

/// Casts a number from one type to another.
pub fn cast<T: std::num::NumCast, U: std::num::NumCast>(n: T) -> U
{
    std::num::cast(n).unwrap()
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
    Bounded::max_value()
}

pub fn min<T: Bounded>() -> T
{
    Bounded::min_value()
}


