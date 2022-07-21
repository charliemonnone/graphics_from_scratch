use macroquad::prelude::Color;

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

#[derive(Debug, Default, Clone, Copy)]
pub struct Point2 {
	pub x: i32,
	pub y: i32,
	pub h: f32
}

impl Point2 {

	pub fn add(self, x: i32, y: i32) -> Self {
		Self::new(self.x + x, self.y + y, self.h)
	}
	
	pub const fn new(x: i32, y: i32, h: f32) -> Self {
		Self { x, y, h }
	}

}

#[derive(Debug, Default, Copy, Clone)]
pub struct Triangle {
	pub v0: u32,
	pub v1: u32,
	pub v2: u32,
	pub color: Color
}

impl Triangle {
	pub const fn new(v0: u32, v1: u32, v2: u32, color: Color) -> Self {
		Self { v0, v1, v2, color }
	}
}