#![cfg_attr(feature="int_roundings",feature(int_roundings))]
pub trait IsZero { fn is_zero(&self) -> bool; }

pub trait Zero { const ZERO: Self; }
impl Zero for u8 { const ZERO: Self = 0; }
impl Zero for u16 { const ZERO: Self = 0; }
impl Zero for u32 { const ZERO: Self = 0; }
impl Zero for u64 { const ZERO : Self = 0; }
impl Zero for usize { const ZERO : Self = 0; }
impl Zero for i8 { const ZERO: Self = 0; }
impl Zero for i16 { const ZERO: Self = 0; }
impl Zero for i32 { const ZERO : Self = 0; }
impl Zero for f32 { const ZERO : Self = 0.; }
impl Zero for f64 { const ZERO : Self = 0.; }
impl Zero for std::sync::atomic::AtomicU16 { const ZERO : Self = Self::new(0); }
impl<T0:Zero,T1:Zero> Zero for (T0,T1) { const ZERO : Self = (Zero::ZERO,Zero::ZERO); }
impl<T: Zero, const N: usize> Zero for [T; N] { const ZERO : Self = [Zero::ZERO; N]; }
impl<T: Zero> Zero for std::ops::Range<T> { const ZERO : Self = Zero::ZERO..Zero::ZERO; }
impl<T:Zero+PartialEq> IsZero for T { fn is_zero(&self) -> bool { self == &Zero::ZERO} }

pub const fn zero<T:Zero>() -> T { T::ZERO }

pub trait Option<T> { fn unwrap_or_zero(self) -> T; }
impl<T:Zero> Option<T> for std::option::Option<T> { fn unwrap_or_zero(self) -> T { self.unwrap_or(zero()) }}

pub trait Signed { fn signum(&self) -> Self; fn abs(&self) -> Self; }
macro_rules! signed_impl { ($($T:ty)+) => ($( impl Signed for $T { fn signum(&self) -> Self { <$T>::signum(*self) } fn abs(&self) -> Self { <$T>::abs(*self) } } )+) }
signed_impl!(i16 i32 f32 f64);
pub fn sign<T:Signed>(x : T) -> T { x.signum() }
pub fn abs<T:Signed>(x : T) -> T { x.abs() }

use std::ops::Mul;
pub fn sq<T:Copy+Mul>(x: T) -> T::Output { x*x }
pub fn cb<T:Copy+Mul>(x: T) -> <T::Output as std::ops::Mul<T>>::Output where <T as std::ops::Mul>::Output : std::ops::Mul<T> { x*x*x }
pub fn pow(x: f64, k: f64) -> f64 { f64::powf(x, k) }
pub fn powi(x: f64, k: i32) -> f64 { f64::powi(x, k) }

pub fn floor(x : f32) -> f32 { x.floor() }
pub fn fract(x: f32) -> f32 { x.fract() }

pub trait Lerp<T> { fn lerp(&self, a: T, b: T) -> T; }
pub fn lerp<T>(t: f32, a: T, b: T) -> T where f32:Lerp<T> { Lerp::lerp(&t, a, b) }
impl Lerp<f32> for f32 { fn lerp(&self, a: f32, b: f32) -> f32 { let t=*self; assert!(t >= 0. && t<= 1.); (1.-t)*a + t*b } }
impl Lerp<u32> for f32 { fn lerp(&self, a: u32, b: u32) -> u32 { let t=*self; assert!(t >= 0. && t<= 1.); f32::round((1.-t)*a as f32 + t*b as f32) as u32 } }
impl Lerp<u8> for f32 { fn lerp(&self, a: u8, b: u8) -> u8 { let t=*self; assert!(t >= 0. && t<= 1.); f32::round((1.-t)*a as f32 + t*b as f32) as u8 } }

pub trait Sqrt { fn sqrt(self) -> Self; }
impl Sqrt for f32 { #[track_caller] fn sqrt(self) -> Self { assert!(self >= 0., "{}", self); Self::sqrt(self) } }
impl Sqrt for f64 { #[track_caller] fn sqrt(self) -> Self { assert!(self >= 0., "{}", self); Self::sqrt(self) } }
#[track_caller] pub fn sqrt<T:Sqrt>(x: T) -> T { x.sqrt() }

pub trait Log { fn log2(self) -> Self; }
impl Log for f32 { #[track_caller] fn log2(self) -> Self { assert!(self >= 0., "{}", self); Self::log2(self) } }
impl Log for f64 { #[track_caller] fn log2(self) -> Self { assert!(self >= 0., "{}", self); Self::log2(self) } }
#[track_caller] pub fn log2<T:Log>(x: T) -> T { x.log2() }

pub fn ln(x: impl std::borrow::Borrow<f64>) -> f64 { f64::ln(*x.borrow()) }

pub fn cos(x: f32) -> f32 { x.cos() }
pub fn sin(x: f32) -> f32 { x.sin() }
pub fn atan(y: f32, x: f32) -> f32 { y.atan2(x) }
pub fn exp10(x: f64) -> f64 { f64::exp(f64::ln(10.)*x) }

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)] pub struct Ratio { pub num: u32, pub div: u32 }
#[allow(non_upper_case_globals)] pub const unit : Ratio = Ratio{num: 1, div: 1};
#[allow(non_upper_case_globals)] pub const undefined : Ratio = Ratio{num: 0, div: 0};
impl Default for Ratio { fn default() -> Self { unit } }
impl Ratio {
	pub fn rcp(&self) -> Ratio { Self{num: self.div, div: self.num} }
	#[cfg(feature="int_roundings")] pub fn ceil(&self, x: u32) -> u32 { u32::div_ceil(x * self.num, self.div) }
	#[cfg(feature="int_roundings")] pub fn ifloor(&self, x: i32) -> i32 { i32::div_floor(x * self.num as i32, self.div as i32) }
	#[cfg(feature="int_roundings")] pub fn iceil(&self, x: i32) -> i32 { i32::div_ceil(x * self.num as i32, self.div as i32) }
}
impl From<Ratio> for f32 { fn from(r: Ratio) -> Self { r.num as f32 / r.div as f32 } }
#[cfg(feature="int_roundings")] impl std::ops::Mul<u32> for Ratio { type Output=u32; fn mul(self, b: u32) -> Self::Output { u32::div_floor(b * self.num, self.div) } }
#[cfg(feature="int_roundings")] impl std::ops::Mul<Ratio> for u32 { type Output=u32; fn mul(self, b: Ratio) -> Self::Output { b*self } }
#[cfg(feature="int_roundings")] impl std::ops::Mul<i32> for Ratio { type Output=i32; fn mul(self, b: i32) -> Self::Output { self.ifloor(b) } }
impl std::ops::Mul<Ratio> for Ratio { type Output=Ratio; fn mul(self, b: Ratio) -> Self::Output { Ratio{num: self.num * b.num, div: self.div * b.div} } }
#[cfg(feature="int_roundings")] impl std::ops::Div<Ratio> for u32 { type Output=u32; #[track_caller] fn div(self, r: Ratio) -> Self::Output { u32::div_floor(self * r.div, r.num) } }
#[cfg(feature="int_roundings")] impl std::ops::Div<Ratio> for i32 { type Output=i32; fn div(self, r: Ratio) -> Self::Output { i32::div_floor(self * r.div as i32, r.num as i32) } }
impl std::ops::Mul<f32> for Ratio { type Output=f32; fn mul(self, b: f32) -> Self::Output { b * self.num as f32 / self.div as f32 } } // loses precision
impl std::ops::Div<Ratio> for f32 { type Output=f32; fn div(self, r: Ratio) -> Self::Output { self * r.div as f32 / r.num as f32 } } // loses precision
impl std::cmp::PartialOrd<Ratio> for Ratio { fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> { Some(self.cmp(other)) } }
impl std::cmp::Ord for Ratio { fn cmp(&self, other: &Self) -> std::cmp::Ordering { (self.num*other.div).cmp(&(other.num*self.div)) } }

pub fn relative_error(a: f64, b: f64) -> f64 {
    if a==0. && b==0. { 0. }
    else if sign(a) != sign(b) { f64::INFINITY }
    else { abs(b-a)/abs(a).min(abs(b)) }
}

pub trait IsOne { fn is_one(&self) -> bool; }
pub trait One { const ONE: Self; }
impl One for u8 { const ONE: Self = 1; }
impl One for i8 { const ONE : Self = 1; }
impl One for f64 { const ONE : Self = 1.; }
impl<T:One+PartialEq> IsOne for T { fn is_one(&self) -> bool { self == &One::ONE} }

pub trait IsMinusOne { fn is_minus_one(&self) -> bool; }
pub trait MinusOne { const MINUS_ONE: Self; }
impl MinusOne for i8 { const MINUS_ONE: Self = -1; }
impl MinusOne for f64 { const MINUS_ONE : Self = -1.; }
impl<T:MinusOne+PartialEq> IsMinusOne for T { fn is_minus_one(&self) -> bool { self == &MinusOne::MINUS_ONE} }
impl IsMinusOne for u8 { fn is_minus_one(&self) -> bool { false } }
