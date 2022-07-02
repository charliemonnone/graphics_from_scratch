use crate::{color::Color24, point::Point3};

#[derive(Debug)]
pub struct Sphere {
	pub center: Point3,
	pub radius: f32,
	pub color: Color24
}

impl Default for Sphere {
	/// Returns a red sphere at 0,0,0
	fn default() -> Self { Self { center: Point3::default(), radius: 1.0, color: Color24::new(255, 0, 0) }}
}

impl Sphere {
	pub fn new(center: Point3, radius: f32, color: Color24) -> Self {
		Self { center, radius, color }
	}
}