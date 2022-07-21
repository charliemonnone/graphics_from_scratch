use super::vec3::Position;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
	pub pos: Position,
	pub viewport_dist: f32,
	pub view_width: f32,
	pub view_height: f32,
}

impl Camera {
	pub fn new(pos: Position, viewport_dist: f32, view_width: f32, view_height: f32) -> Self {
		Self { pos, viewport_dist, view_width, view_height }
	}
}

impl Default for Camera {
	fn default() -> Self {
		Self {
			viewport_dist: 1.0,
			pos: Position::default(),
			view_height: 1.0,
			view_width: 1.0,
		}

	}
}