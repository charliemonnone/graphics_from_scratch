use super::vec3::Vec3;
use std::cmp::{max, min};
pub use std::f32::INFINITY;

pub fn sqrt_f32(n: f32) -> f32 {
    n.sqrt()
}

pub fn clamp_u8(n: u8) -> u8 {
    min(u8::max_value(), max(0, n))
}

pub fn vec_length(v: &Vec3<f32>) -> f32 {
    sqrt_f32(v.length_squared())
}

pub fn pow(n: f32, p: f32) -> f32 {
    n.powf(p)
}
