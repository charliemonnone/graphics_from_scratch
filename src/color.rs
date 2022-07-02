use macroquad::prelude::Color;

use crate::math;

#[derive(Debug, Default, Copy, Clone)]
pub struct Color24 {
	pub r: u8,
	pub g: u8,
	pub b: u8
}

impl Color24 {
	pub fn new(r: u8, g: u8, b:u8) -> Color24 {
		Color24 { r, g, b }
	}

	pub fn mult(&mut self, c: u8) -> Color24 {
		let mut color = Color24::new(self.r*c, self.g*c, self.b*c);
		clamp(&mut color);
		
		color
	}

	pub fn add(&mut self, color: Color24) -> Color24 {
		let mut color = Color24::new(self.r + color.r, self.g + color.g, self.b + color.b); 
		clamp(&mut color);

		color
	}

	fn to_string(&self, samples_per_pixel: i64) -> String {
		let mut r: f64 = self.r as f64;
		let mut g: f64 = self.g as f64;
		let mut b: f64 = self.b as f64;
	
		let scale = 1.0 / (samples_per_pixel as f64);
		r *= scale;
		g *= scale;
		b *= scale;
	
		let x = (256.0 * math::clamp_f64(r, 0.0, 0.999)) as i64;
		let y = (256.0 * math::clamp_f64(g, 0.0, 0.999)) as i64;
		let z = (256.0 * math::clamp_f64(b, 0.0, 0.999)) as i64;
	
		format!("{} {} {}\n", x, y, z)
	}
}

fn clamp(color: &mut Color24) {
	color.r = math::clamp_u8(color.r);
	color.g = math::clamp_u8(color.g);
	color.b = math::clamp_u8(color.b);
}
