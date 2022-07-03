use std::ops::{Add, Div, Mul, Sub, Rem, AddAssign, SubAssign, MulAssign};
use crate::math;
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3<T> {
    pub v0: T,
    pub v1: T,
    pub v2: T
}

pub type Point3 = Vec3<f32>;

// Trait bound "alias" hack
// https://www.worthe-it.co.za/blog/2017-01-15-aliasing-traits-in-rust.html
pub trait VecData: 
    Add<Output=Self> 
	+ Sub<Output=Self> 
	+ Mul<Output=Self> 
	+ Div<Output=Self>
	+ Rem<Output=Self> 
	+ AddAssign
	+ SubAssign
	+ MulAssign
	+ Copy 
	+ PartialEq 
	+ PartialOrd
    where Self: std::marker::Sized {}

impl <T> VecData for T where T:
	Add<Output=Self> 
	+ Sub<Output=Self> 
	+ Mul<Output=Self> 
	+ Div<Output=Self>
	+ Rem<Output=Self> 
	+ AddAssign
	+ SubAssign
	+ MulAssign
	+ Copy 
	+ PartialEq 
	+ PartialOrd {
	} 

/*
	impl operations will mutate self when applicable
	allocating versions of these operations are implemented below
 	this impl block
*/
impl<T> Vec3<T> where T: VecData {
	pub const fn new(v0: T, v1: T, v2: T) -> Self {
		Self { v0, v1, v2}
	}

	pub fn sub(&mut self, other: &Self) {
		self.v0 -= other.v0;
		self.v1 -= other.v2;
		self.v2 -= other.v2;
	}

	pub fn add(&mut self, other: &Self) {
		self.v0 += other.v0;
		self.v1 += other.v1; 
		self.v2 += other.v2;
	}

	pub fn mul(&mut self, other: &Self) {
		self.v0 *= other.v0;
		self.v1 *= other.v1;
		self.v2 *= other.v2;
	}

	pub fn scale(&mut self, scalar: T) {
		self.v0 *= scalar;
		self.v1 *= scalar;
		self.v2 *= scalar;
	}

	pub fn length_squared(&self) -> T {
		(self.v0 * self.v0) 
		+ (self.v1 * self.v1) 
		+ (self.v2 * self.v2)
	}
} 

pub fn sub<T: VecData> (u: &Vec3<T>, v: &Vec3<T>) -> Vec3<T> {
	Vec3::new(
		u.v0 - v.v0,
		u.v1 - v.v1,
		u.v2 - v.v2,
	)
}

pub fn add<T: VecData> (u: &Vec3<T>, v: &Vec3<T>) -> Vec3<T> {
	Vec3::new(
		u.v0 + v.v0, 
		u.v1 + v.v1, 
		u.v2 + v.v2
	)
}

pub fn mul<T: VecData> (u: &Vec3<T>, v: &Vec3<T>) -> Vec3<T> {
	Vec3::new(
		u.v0 * v.v0,
		u.v1 * v.v1,
		u.v2 * v.v2,
	)
}

pub fn dot<T: VecData> (u: &Vec3<T>, v: &Vec3<T>) -> T {
	(u.v0 * v.v0)  
	+ (u.v1 * v.v1)  
	+ (u.v2 * v.v2)
}

pub fn cross<T: VecData> (u: &Vec3<T>, v: &Vec3<T>) -> Vec3<T> {
	Vec3::new(
		(u.v1 * v.v2) - (u.v2 * v.v1), 
		(u.v2 * v.v0) - (u.v0 * v.v2), 
		(u.v0 * v.v1) - (u.v1 * v.v0),
	)
}

pub fn scale<T: VecData>(u: &Vec3<T>, scalar: T) -> Vec3<T> {
	Vec3::new(
		u.v0 * scalar,
		u.v1 * scalar,
		u.v2 * scalar
	)
}

pub fn length_squared<T: VecData> (u: &Vec3<T>) -> T {
	(u.v0 * u.v0) 
	+ (u.v1 * u.v1) 
	+ (u.v2 * u.v2)
}