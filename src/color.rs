pub use macroquad::prelude::Color;

use crate::math;
pub const RED:   Color = Color::new(255.0, 0.0, 0.0, 255.0);
pub const BLUE:  Color = Color::new(0.0, 0.0, 255.0, 255.0);
pub const GREEN: Color = Color::new(0.0, 255.0, 0.0, 255.0);
pub const WHITE: Color = Color::new(255.0, 255.0, 255.0, 255.0);
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 255.0);

#[derive(Debug, Default, Copy, Clone)]
pub struct Color24 {
	pub r: u8,
	pub g: u8,
	pub b: u8
}