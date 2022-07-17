use macroquad::prelude::{screen_width, screen_height};

#[derive(Debug, Default, Clone, Copy)]
pub struct Point2 {
	pub x: i32,
	pub y: i32,
	pub h: f32
}

impl Point2 {
	pub fn centered(h: Option<f32>) -> Self {
		
		let hue = if let Some(hue) = h {
			hue
		} else {
			1.0
		};

		Self {
			x: (screen_width() / 2.) as i32,
			y: (screen_height() / 2.) as i32,
			h: hue
		}

	}

	pub fn add(self, x: i32, y: i32) -> Self {
		Self::new(self.x + x, self.y + y, self.h)
	}
	
	pub fn new(x: i32, y: i32, h: f32) -> Self {
		Self { x, y, h }
	}

}