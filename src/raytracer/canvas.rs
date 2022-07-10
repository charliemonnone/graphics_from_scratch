use crate::raytracer::{color::Color24, vec3::Point};

#[derive(Debug)]
pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pub origin: Point,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            origin: Point::default(),
        }
    }
}
