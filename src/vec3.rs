use std::{ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, Sub, SubAssign, Neg, Index, IndexMut}};

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3<T> {
    pub v0: T,
    pub v1: T,
    pub v2: T,
	sentintel: T
}

pub type Point = Vec3<f32>;

impl<T: VecData> Index<usize> for Vec3<T> {
	type Output = T;
	fn index(&self, i: usize) -> &Self::Output {
		match i {
			0 => &self.v0,
			1 => &self.v1,
			2 => &self.v2,
			_ => &self.sentintel
		}
	}
}

impl<T: VecData> IndexMut<usize> for Vec3<T> {
	fn index_mut(&mut self, i: usize) -> &mut T {
		
		match i {
			0 => &mut self.v0,
			1 => &mut self.v1,
			2 => &mut self.v2,
			_ => &mut self.sentintel
		}
	}
}

// Trait bound "alias" hack
// https://www.worthe-it.co.za/blog/2017-01-15-aliasing-traits-in-rust.html
pub trait VecData:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
	+ Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + Copy
    + PartialEq
    + PartialOrd
	+ Default
where
    Self: std::marker::Sized, {}

impl<T> VecData for T where
    T: Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
		+ Neg<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + Copy
        + PartialEq
        + PartialOrd 
		+ Default {}

/*
    impl operations will mutate self when applicable
    allocating versions of these operations are implemented below
    for various operators
*/
impl<T> Vec3<T>
where T: VecData {
    pub fn new(v0: T, v1: T, v2: T) -> Self {
        Self { v0, v1, v2, sentintel: T::default() }
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
        (self.v0 * self.v0) + (self.v1 * self.v1) + (self.v2 * self.v2)
    }
}

pub fn neg<T: VecData> (u: &Vec3<T>) -> Vec3<T> {
	Vec3::new(-u.v0, -u.v1, -u.v2)
}

pub fn dot<T: VecData> (u: &Vec3<T>, v: &Vec3<T>) -> T {
    (u.v0 * v.v0) + (u.v1 * v.v1) + (u.v2 * v.v2)
}

pub fn _cross<T: VecData>(u: &Vec3<T>, v: &Vec3<T>) -> Vec3<T> {
    Vec3::new(
        (u.v1 * v.v2) - (u.v2 * v.v1),
        (u.v2 * v.v0) - (u.v0 * v.v2),
        (u.v0 * v.v1) - (u.v1 * v.v0),
    )
}

impl<T: VecData> Add for Vec3<T> {
	type Output = Vec3<T>;
	fn add(self, other: Self) -> Self::Output {
		Vec3::new(
			self.v0 + other.v0,
			self.v1 + other.v1,
			self.v2 + other.v2,
		)
	}
}

impl<T: VecData> Add for &Vec3<T> {
	type Output = Vec3<T>;
	fn add(self, other: Self) -> Self::Output {
		Vec3::new(
			self.v0 + other.v0,
			self.v1 + other.v1,
			self.v2 + other.v2,
		)
	}
}

impl<T: VecData> Sub for Vec3<T> {
	type Output = Vec3<T>;
	fn sub(self, other: Self) -> Self::Output {
		Vec3::new(
			self.v0 - other.v0,
			self.v1 - other.v1,
			self.v2 - other.v2,
		)
	}
}

impl<T: VecData> Sub for &Vec3<T> {
	type Output = Vec3<T>;
	fn sub(self, other: Self) -> Self::Output {
		Vec3::new(
			self.v0 - other.v0,
			self.v1 - other.v1,
			self.v2 - other.v2,
		)
	}
}

impl<T: VecData> Neg for Vec3<T> {
	type Output = Vec3<T>;
	fn neg(self) -> Self::Output {
		Vec3::new(
			-self.v0,
			-self.v1,
			-self.v2,
		)
	}
}

impl<T: VecData> Neg for &Vec3<T> {
	type Output = Vec3<T>;
	fn neg(self) -> Self::Output {
		Vec3::new(
			-self.v0,
			-self.v1,
			-self.v2,
		)
	}
}

impl<T: VecData> Mul<T> for Vec3<T> {
	type Output = Vec3<T>;
	fn mul(self, other: T) -> Self::Output {
		Vec3::new(
			self.v0 * other,
			self.v1 * other,
			self.v2 * other,
		)
	}
}

impl<T: VecData> Mul<T> for &Vec3<T> {
	type Output = Vec3<T>;
	fn mul(self, other: T) -> Self::Output {
		Vec3::new(
			self.v0 * other,
			self.v1 * other,
			self.v2 * other,
		)
	}
}