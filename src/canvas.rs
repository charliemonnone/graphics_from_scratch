use crate::color::Color24;
use crate::vec3::Point3;

const DEFAULT_VIEW_WIDTH: i32 = 800;
const DEFAULT_VIEW_HEIGHT: i32 = 600;

#[derive(Debug)]
pub struct Canvas {
	pub width: i32,
	pub height: i32,
	pub origin: Point3
}

impl Canvas {
	pub fn new(width: i32, height: i32) -> Self {
		Self { width, height, origin: Point3::default() }
	}
}