#![feature(min_const_generics, const_in_array_repeat_expressions, const_fn, in_band_lifetimes)]

pub trait IsZero { fn is_zero(&self) -> bool; }

pub trait Zero { const ZERO: Self; }
impl Zero for u32 { const ZERO: Self = 0; }
impl Zero for u64 { const ZERO : Self = 0; }
impl Zero for usize { const ZERO : Self = 0; }
impl Zero for i32 { const ZERO : Self = 0; }
impl Zero for f32 { const ZERO : Self = 0.; }
impl Zero for f64 { const ZERO : Self = 0.; }
impl<T0:Zero,T1:Zero> Zero for (T0,T1) { const ZERO : Self = (Zero::ZERO,Zero::ZERO); }
impl<T: Zero, const N: usize> Zero for [T; N] { const ZERO : Self = [Zero::ZERO; N]; }

impl<T:Zero+PartialEq> IsZero for T { fn is_zero(&self) -> bool { self == &Zero::ZERO} }

pub const fn zero<T:Zero>() -> T { T::ZERO }

pub trait Signed { fn signum(&self) -> Self; fn abs(&self) -> Self; }
macro_rules! signed_impl { ($($T:ty)+) => ($( impl Signed for $T { fn signum(&self) -> Self { <$T>::signum(*self) } fn abs(&self) -> Self { <$T>::abs(*self) } } )+) }
signed_impl!(i16 i32 f32);
pub fn sign<T:Signed>(x : T) -> T { x.signum() }
pub fn abs<T:Signed>(x : T) -> T { x.abs() }
pub fn sq<T:Copy+std::ops::Mul>(x: T) -> T::Output { x*x }
pub fn cb<T:Copy+std::ops::Mul>(x: T) -> <T::Output as std::ops::Mul<T>>::Output where <T as std::ops::Mul>::Output : std::ops::Mul<T> { x*x*x }

pub fn div_floor(n : u32, d : u32) -> u32 { n/d }
pub fn div_ceil(n : u32, d : u32) -> u32 { (n+d-1)/d }

pub fn idiv_rem(n : i32, d : u32) -> (i32, i32) { (n/d as i32, n%d as i32) }
pub fn idiv_floor(n: i32, d: u32) -> i32 {
	let (q, r) = idiv_rem(n, d);
	if r < 0 { q - 1 } else { q }
}
pub fn idiv_ceil(n: i32, d: u32) -> i32 {
	let (q, r) = idiv_rem(n, d);
	if r > 0 { q + 1 } else { q }
}

pub fn floor(x : f32) -> f32 { x.floor() }
pub fn fract(x: f32) -> f32 { x.fract() }
pub fn sqrt(x: f32) -> f32 { x.sqrt() }
pub fn cos(x: f32) -> f32 { x.cos() }
pub fn sin(x: f32) -> f32 { x.sin() }
pub fn atan(y: f32, x: f32) -> f32 { y.atan2(x) }

pub fn clamp<T:PartialOrd>(min: T, x: T, max: T) -> T { if x < min {min} else if x > max {max} else {x} }

#[derive(Clone,Copy,Debug,PartialEq)] pub struct Ratio { pub num: u32, pub div: u32 }
impl Default for Ratio { fn default() -> Self { Self{num: 1, div: 1} } }
impl Ratio {
	pub fn ceil(&self, x: u32) -> u32 { div_ceil(x * self.num, self.div) }
	pub fn ifloor(&self, x: i32) -> i32 { idiv_floor(x * self.num as i32, self.div) }
	pub fn iceil(&self, x: i32) -> i32 { idiv_ceil(x * self.num as i32, self.div) }
}
impl From<Ratio> for f32 { fn from(r: Ratio) -> Self { r.num as f32 / r.div as f32 } }
impl std::ops::Mul<u32> for Ratio { type Output=u32; fn mul(self, b: u32) -> Self::Output { div_floor(b * self.num, self.div) } }
impl std::ops::Div<Ratio> for u32 { type Output=u32; fn div(self, r: Ratio) -> Self::Output { div_floor(self * r.div, r.num) } }
impl std::ops::Div<Ratio> for i32 { type Output=i32; fn div(self, r: Ratio) -> Self::Output { idiv_floor(self * r.div as i32, r.num) } }
impl std::ops::Mul<f32> for Ratio { type Output=f32; fn mul(self, b: f32) -> Self::Output { b * self.num as f32 / self.div as f32 } } // loses precision
impl std::ops::Div<Ratio> for f32 { type Output=f32; fn div(self, r: Ratio) -> Self::Output { self * r.div as f32 / r.num as f32 } } // loses precision

#[allow(non_camel_case_types)] #[derive(PartialEq,Clone,Copy,PartialOrd,Debug,serde::Deserialize)] pub struct real(pub f32);

impl std::cmp::Eq for real {}

impl Zero for real { const ZERO : Self = real(Zero::ZERO); }

impl std::ops::Neg for real { type Output = Self; fn neg(self) -> Self { real(-self.0) } }
impl std::ops::Neg for &real { type Output = real; fn neg(self) -> real { real(-self.0) } }

impl std::ops::Add<Self> for real { type Output = Self; fn add(self, b: Self) -> Self { real(self.0.add(b.0)) } }
impl std::ops::AddAssign<Self> for real { fn add_assign(&mut self, b: Self) { self.0.add_assign(b.0) } }

impl std::iter::Sum<Self> for real { fn sum<I:Iterator<Item=Self>>(iter: I) -> Self { real(iter.map(|real(f)| f).sum()) } }
impl std::iter::Sum<&'t Self> for real { fn sum<I:Iterator<Item=&'t Self>>(iter: I) -> Self { real(iter.map(|real(f)| f).sum()) } }

impl std::ops::Sub<Self> for real { type Output = Self; fn sub(self, b: Self) -> Self { real(self.0.sub(b.0)) } }
impl std::ops::Sub<&real> for real { type Output = Self; fn sub(self, b: &real) -> Self { real(self.0.sub(b.0)) } }
impl std::ops::Sub<real> for &real { type Output = real; fn sub(self, b: real) -> Self::Output { real(self.0.sub(b.0)) } }
impl std::ops::Sub<&real> for &real { type Output = real; fn sub(self, b: &real) -> Self::Output { real(self.0.sub(b.0)) } }

impl std::cmp::Ord for real { fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.partial_cmp(other).unwrap() } }

impl std::ops::Mul<Self> for real { type Output = Self; fn mul(self, b: Self) -> Self { real(self.0.mul(b.0)) } }
impl std::ops::Mul<&real> for real { type Output = Self; fn mul(self, b: &real) -> Self { real(self.0.mul(b.0)) } }
impl std::ops::Mul<real> for &real { type Output = real; fn mul(self, b: real) -> Self::Output { real(self.0.mul(b.0)) } }
impl std::ops::Mul<&real> for &real { type Output = real; fn mul(self, b: &real) -> Self::Output { real(self.0.mul(b.0)) } }

impl std::iter::Product<Self> for real { fn product<I:Iterator<Item=Self>>(iter: I) -> Self { real(iter.map(|real(f)| f).product()) } }
impl std::iter::Product<&'t Self> for real { fn product<I:Iterator<Item=&'t Self>>(iter: I) -> Self { real(iter.map(|real(f)| f).product()) } }

impl std::ops::Div<Self> for real { type Output = Self; #[track_caller] fn div(self, b: Self) -> Self { assert!(!IsZero::is_zero(&b)); real(self.0.div(b.0)) } }
impl std::ops::Div<real> for &real { type Output = real; #[track_caller] fn div(self, b: real) -> Self::Output { assert!(!IsZero::is_zero(&b)); real(self.0.div(b.0)) } }
impl std::ops::Div<&real> for real { type Output = Self; #[track_caller] fn div(self, b: &real) -> Self { assert!(!IsZero::is_zero(b)); real(self.0.div(b.0)) } }

impl std::fmt::LowerExp for real { fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(fmt) } }

impl real {
	pub fn abs(self) -> real { real(f32::abs(self.0)) }
	pub fn recip(self) -> real { real(f32::recip(self.0)) }
	pub fn powi(self, n: i32) -> real { real(f32::powi(self.0, n)) }
	pub fn floor(self) -> real { real(f32::floor(self.0)) }
	pub fn ceil(self) -> real { real(f32::ceil(self.0)) }
	pub fn exp2(self) -> real { real(f32::exp2(self.0)) }
	pub fn exp(self) -> real { real(f32::exp(self.0)) }
	pub fn exp10(self) -> real { real::exp(real::ln(real(10.))*self) }
	pub fn pow(self, n: real) -> real { real(f32::powf(self.0, n.0)) }
	#[track_caller] pub fn ln(self) -> real { assert!(self>zero(),"ln {:?}", self); real(f32::ln(self.0)) }
	#[track_caller] pub fn log10(self) -> real { assert!(self>zero(),"log10 {:?}", self); real(f32::log10(self.0)) }
}
