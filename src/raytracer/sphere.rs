use crate::raytracer::{
    color::{Color, RED},
    vec3::Point,
};

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub radius_sq: f32,
    pub color: Color,
    pub specular: f32,
    pub reflective: f32,
}

impl Default for Sphere {
    /// Returns a red sphere at 0,0,0
    fn default() -> Self {
        Self {
            center: Point::default(),
            radius: 1.0,
            radius_sq: 1.0 * 1.0,
            color: RED,
            specular: 10.0,
            reflective: 0.5,
        }
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f32, color: Color, specular: f32, reflective: f32) -> Self {
        Self {
            center,
            radius,
            radius_sq: radius * radius,
            color,
            specular,
            reflective,
        }
    }
}
