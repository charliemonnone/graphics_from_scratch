use crate::point::Point3;
pub use std::{f32::INFINITY};
use std::cmp::{min, max};

pub fn dot(u: &Point3, v: &Point3) -> f32 {
	(
		(u.x * v.x)  
		+ (u.y * v.y)  
		+ (u.z * v.z)
	) as f32  
}

pub fn sub(u: &Point3, v: &Point3) -> Point3 {
	Point3::new(
		u.x - v.x,
		u.y - v.y,
		u.z - v.z
	)
}

pub fn cross(u: &Point3, v: &Point3) -> Point3 {
	Point3::new(
		u.y * v.z - u.z * v.y,
		u.z * v.x - u.x * v.z,
		u.x * v.y - u.y * v.x
	)
}

pub fn sqrt(n: f32) -> f32 {
	n.sqrt()
}

pub fn clamp_u8(n: u8) -> u8  {
	min(255, max(0, n))
}

pub fn clamp_f64(x: f64, min: f64, max: f64) -> f64 {
	if x < min { return min;}
    if x > max { return max;}
    x
}
