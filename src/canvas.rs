use crate::color::Color24;
use crate::point::Point3;

const DEFAULT_VIEW_WIDTH: i32 = 800;
const DEFAULT_VIEW_HEIGHT: i32 = 600;

#[derive(Debug)]
pub struct Canvas {
	pub width: i32,
	pub height: i32,
	pub origin: Point3
}

impl Default for Canvas {
	fn default() -> Self { Canvas { width: DEFAULT_VIEW_WIDTH, height: DEFAULT_VIEW_HEIGHT, origin: Point3::default() } }
}

impl Canvas {
	pub fn new(width: i32, height: i32) -> Self {
		Self { width, height, origin: Point3::default() }
	}

	pub fn put_pixel(&mut self, x: i32, y: i32, color: Color24) {
// import renderer

	}

}