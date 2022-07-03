pub use std::{f32::INFINITY};
use std::{cmp::{min, max}};
use crate::vec3::Vec3;

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

pub fn vec_length(v: &Vec3<f32>) -> f32 {
	sqrt(v.length_squared())
	
}