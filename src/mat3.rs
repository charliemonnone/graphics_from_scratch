use std::cmp::Ordering;

use crate::vec3::Vec3;
#[derive(Debug, Default, Clone)]
/// 3x3 matrix
pub struct Mat3 {
	pub data: Vec<f32>,
	rows: usize,
	cols: usize, 
	expected_size: usize
}

impl Mat3 {
	pub fn new(mut data: Vec<f32>) -> Self {
		let expected_size: usize = 9;
		let rows: usize = 3;
		let cols: usize = 3;
		let len = data.len();
		match len.cmp(&expected_size) {
			Ordering::Less => data.resize(expected_size, 0.0),
			Ordering::Greater => data.shrink_to(expected_size),
			Ordering::Equal => {},
		}

		Self { data, rows, cols, expected_size }
	}

	pub fn mul_vec3(&self, other: &Vec3<f32>) -> Vec3<f32> {
		let mut result: Vec3<f32> = Vec3::default();
		let width = self.cols;
		for y in 0..self.rows {
			for x in 0..self.cols {
				let index = (y * width) + x;
				result[y] += other[x] * self.data[index];
			}
		}

		result
	}
}