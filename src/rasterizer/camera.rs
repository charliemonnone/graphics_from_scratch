use super::{vec3::Position, data_types::Mat4x4};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
	pub pos: Position,
	pub orientation: Mat4x4,
	pub viewport_dist: f32,
	pub view_width: f32,
	pub view_height: f32,
}

impl Camera {
	pub fn new(pos: Position, orientation: Mat4x4, viewport_dist: f32, view_width: f32, view_height: f32) -> Self {
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
			pos: Position::default(),
			orientation: Mat4x4::default(),
			viewport_dist: 1.0,
			view_height: 1.0,
			view_width: 1.0,
		}

	}
}