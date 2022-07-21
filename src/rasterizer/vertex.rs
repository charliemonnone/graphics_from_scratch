use std::ops::Add;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Vertex {
	pub const fn new(x: f32, y: f32, z: f32) -> Self {
		Self {x, y, z}
	}
}
