use std::{ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, Sub, SubAssign, Neg, Index, IndexMut}};

use super::utils::math;
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub v0: f32,
    pub v1: f32,
    pub v2: f32,
}

pub type Position = Vec3;

impl Index<usize> for Vec3 {
	type Output = f32;
	fn index(&self, i: usize) -> &Self::Output {
		match i {
			0 => &self.v0,
			1 => &self.v1,
			2 => &self.v2,
			_ => &math::INF

		}
	}
}

impl IndexMut<usize> for Vec3 {
	fn index_mut(&mut self, i: usize) -> &mut f32 {

		match i {
			0 => &mut self.v0,
			1 => &mut self.v1,
			2 => &mut self.v2,
			_ => &mut self.v0
		}
	}
}

impl Vec3 {
    pub fn new(v0: f32, v1: f32, v2: f32) -> Self {
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

    pub fn scale(&mut self, scalar: f32) {
        self.v0 *= scalar;
        self.v1 *= scalar;
        self.v2 *= scalar;
    }

    pub fn length_squared(&self) -> f32 {
        (self.v0 * self.v0) + (self.v1 * self.v1) + (self.v2 * self.v2)
    }
}

pub fn neg(u: &Vec3) -> Vec3 {
	Vec3::new(-u.v0, -u.v1, -u.v2)
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    (u.v0 * v.v0) + (u.v1 * v.v1) + (u.v2 * v.v2)
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        (u.v1 * v.v2) - (u.v2 * v.v1),
        (u.v2 * v.v0) - (u.v0 * v.v2),
        (u.v0 * v.v1) - (u.v1 * v.v0),
    )
}

impl Add for Vec3 {
	type Output = Vec3;
	fn add(self, other: Self) -> Self::Output {
		Vec3::new(
			self.v0 + other.v0,
			self.v1 + other.v1,
			self.v2 + other.v2,
		)
	}
}

impl Add for &Vec3 {
	type Output = Vec3;
	fn add(self, other: Self) -> Self::Output {
		Vec3::new(
			self.v0 + other.v0,
			self.v1 + other.v1,
			self.v2 + other.v2,
		)
	}
}

impl Sub for Vec3 {
	type Output = Vec3;
	fn sub(self, other: Self) -> Self::Output {
		Vec3::new(
			self.v0 - other.v0,
			self.v1 - other.v1,
			self.v2 - other.v2,
		)
	}
}

impl Sub for &Vec3 {
	type Output = Vec3;
	fn sub(self, other: Self) -> Self::Output {
		Vec3::new(
			self.v0 - other.v0,
			self.v1 - other.v1,
			self.v2 - other.v2,
		)
	}
}

impl Neg for Vec3 {
	type Output = Vec3;
	fn neg(self) -> Self::Output {
		Vec3::new(
			-self.v0,
			-self.v1,
			-self.v2,
		)
	}
}

impl Neg for &Vec3 {
	type Output = Vec3;
	fn neg(self) -> Self::Output {
		Vec3::new(
			-self.v0,
			-self.v1,
			-self.v2,
		)
	}
}

impl Mul<f32> for Vec3 {
	type Output = Vec3;
	fn mul(self, other: f32) -> Self::Output {
		Vec3::new(
			self.v0 * other,
			self.v1 * other,
			self.v2 * other,
		)
	}
}

impl Mul<f32> for &Vec3 {
	type Output = Vec3;
	fn mul(self, other: f32) -> Self::Output {
		Vec3::new(
			self.v0 * other,
			self.v1 * other,
			self.v2 * other,
		)
	}
}