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
	
	pub fn new(x: i32, y: i32, h: f32) -> Self {
		Self { x, y, h }
	}

}