use super::{ data_types::{Mat4x4, Vertex3}};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
	pub pos: Vertex3,
	pub orientation: Mat4x4,
	pub viewport_dist: f32,
	pub view_width: f32,
	pub view_height: f32,
}

impl Camera {
	pub fn new(pos: Vertex3, orientation: Mat4x4, viewport_dist: f32, view_width: f32, view_height: f32) -> Self {
		Self { 
			pos, 
			orientation, 
			viewport_dist, 
			view_width, 
			view_height 
		}
	}
}

impl Default for Camera {
	fn default() -> Self {
		Self {
			pos: Vertex3::default(),
			orientation: Mat4x4::default(),
			viewport_dist: 1.0,
			view_height: 1.0,
			view_width: 1.0,
		}

	}
}