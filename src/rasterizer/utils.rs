use super::{camera::Camera, data_types::{Mat4x4}, data_types::{Vec4, Vertex3, Point}, main::get_canvas_dimensions};
use macroquad::color::Color;

pub fn interpolate(i0: f32, d0: f32, i1: f32, d1: f32) -> Vec<f32> {
	// i == independent variable
	// d == dependent variable
	if i0 == i1 {
		return vec![d0];
	}

	let mut values = vec![];
	let a = (d1 - d0) / (i1 - i0) as f32;

	let mut d = d0;
	let start = i0 as i32;
	let end = i1 as i32;
	for _i in start..=end {
		values.push(d);
		d += a;
	}

	values
}

pub fn mul_color(color: &Color, h: f32) -> Color {
	Color::new(color.r * h, color.g * h, color.b * h, color.a)
}

pub fn map_to_pixels(x: i32, y: i32, width: i32, height: i32) -> (u32, u32) {

	let x_mapped = (x + (width / 2)) as u32;
    let y_mapped = (y + (height / 2)) as u32;
    (x_mapped, y_mapped)
}

pub fn viewport_to_canvas(cam: &Camera, x: f32, y: f32) -> Point {
	let (width, height) = get_canvas_dimensions();
	Point::new(x * width / cam.view_width, y * height / cam.view_height, 1.0)
}

pub fn project_vertex(cam: &Camera, v: Vec4) -> Point {
	viewport_to_canvas(cam, v.x * cam.viewport_dist / v.z, v.y * cam.viewport_dist / v.z)
}

pub fn make_rotation_mat(degrees: f32) -> Mat4x4 {
	let cos = math::cos(degrees * math::PI / 180.);
	let sin = math::sin(degrees * math::PI / 180.);

	Mat4x4::from_cols(
		Vec4::new(cos, 0., -sin, 0.), 
		Vec4::new(0., 1., 0., 0.), 
		Vec4::new(sin, 0., cos, 0.), 
		Vec4::new(0., 0., 0., 1.), 
	).transpose()
}

pub fn make_translation_mat(translation: Vertex3) -> Mat4x4 {
	Mat4x4::from_cols(
		Vec4::new(1., 0., 0., translation.x), 
		Vec4::new(0., 1., 0., translation.y), 
		Vec4::new(0., 0., 1., translation.z), 
		Vec4::new(0., 0., 0., 1.), 
	).transpose()
}

pub fn make_scaling_mat(scale: f32) -> Mat4x4 {
	Mat4x4::from_cols(
		Vec4::new(scale, 0., 0., 0.), 
		Vec4::new(0., scale, 0., 0.), 
		Vec4::new(0., 0., scale, 0.), 
		Vec4::new(0., 0., 0., 1.), 
	).transpose()
}

pub fn mul_mv(mat: Mat4x4, vec: Vec4) -> Vec4 {
	mat.mul_vec4(vec)
}

pub fn mul_mm(mat_a: Mat4x4, mat_b: Mat4x4) -> Mat4x4 {
	mat_a.mul_mat4(&mat_b)
}

pub mod math {
	use std::f32;

	pub const PI: f32 = std::f32::consts::PI;

	pub fn abs(n: f32) -> f32 {
		n.abs()
	}

	pub fn cos(n: f32) -> f32 {
		n.cos()
	}

	pub fn sin(n: f32) -> f32 {
		n.sin()
	}

	pub fn floor_f(n: f32) -> f32 {
		n.floor()
	}
}