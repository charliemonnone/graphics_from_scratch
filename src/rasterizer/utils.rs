use super::{camera::Camera, vertex::Vertex, point2::Point2, main::get_canvas_dimensions};
use macroquad::color::Color;

pub fn interpolate(i0: i32, d0: f32, i1: i32, d1: f32) -> Vec<f32> {
	// i == independent variable
	// d == dependent variable
	if i0 == i1 {
		return vec![d0];
	}

	let mut values = vec![];
	let a = (d1 - d0) / (i1 - i0) as f32;

	let mut d = d0;
	for _i in i0..=i1 {
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

pub fn viewport_to_canvas(cam: &Camera, x: f32, y: f32) -> Point2 {
	let (width, height) = get_canvas_dimensions();
	Point2::new((x * width / cam.view_width) as i32 , (y * height/ cam.view_height) as i32, 1.0)
}

pub fn project_vertex(cam: &Camera, v: &Vertex) -> Point2 {
	viewport_to_canvas(cam, v.x * cam.viewport_dist / v.z, v.y * cam.viewport_dist / v.z)
}