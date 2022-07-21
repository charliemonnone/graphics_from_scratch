use std::mem::swap;
use super::{point2::Point2, math, utils, main::get_canvas_dimensions, vertex::Vertex, shapes::Triangle, camera::Camera};
use macroquad::{texture::Image, color::Color};

pub fn draw_line(image: &mut Image, p0: Point2, p1: Point2, color: Color) {
	let mut p0 = p0;
	let mut p1 = p1;

	let mut x0 = p0.x;
	let mut y0 = p0.y;
	let mut x1 = p1.x;
	let mut y1 = p1.y;

	let dx = x1 - x0;
	let dy = y1 - y0;
	
	if math::abs(dx) > math::abs(dy) {
		// horizontalish
		if x0 > x1 { 
			swap(&mut p0, &mut p1); 
			x0 = p0.x;
			y0 = p0.y;
			x1 = p1.x;
			y1 = p1.y;
		}
		let ys = utils::interpolate(x0, y0 as f32, x1, y1 as f32);
		for x in x0..=x1 {
			let index: usize = (x - x0) as usize;
			let y = ys[index];
			put_pixel(image, x, y as i32, color);
		}
	} else {
		// verticalish
		if  y0 > y1 { 
			swap(&mut p0, &mut p1); 
			x0 = p0.x;
			y0 = p0.y;
			x1 = p1.x;
			y1 = p1.y;
		}
		let xs = utils::interpolate(y0, x0 as f32, y1, x1 as f32);

		for y in y0..=y1 {
			let index: usize = (y - y0) as usize;
			let x = xs[index];
			put_pixel(image, x as i32, y, color);
		}
	}
}

/// Given upper left point p0, a width and height, draw a rectangle
pub fn draw_rect(image: &mut Image, p0: Point2, width: i32, height: i32, color: Color, fill: Option<Color>) {
	
	let p1 = Point2::new(p0.x + width, p0.y, p0.h); 
	let p2 = Point2::new(p1.x, p0.y + height, p0.h); 
	let p3 = Point2::new(p0.x, p2.y, p0.h); 
	// p0 --- p1
	// |	  |
	// |	  |
	// p3 --- p2

	if let Some(fill_color) = fill {
		fill_rect(image, p0, width, height, fill_color)
	}

	draw_line(image, p0, p1, color);
	draw_line(image, p1, p2, color);
	draw_line(image, p3, p2, color);
	draw_line(image, p0, p3, color);

}

pub fn draw_triangle(image: &mut Image, p0: Point2, p1: Point2, p2: Point2, stroke: Option<Color>, fill: Option<Color>) {
	if let Some(fill_color) = fill {
		shade_triangle(image, p0, p1, p2, fill_color);
	}

	if let Some(stroke_color) = stroke {
		draw_line(image, p0, p1, stroke_color);
		draw_line(image, p1, p2, stroke_color);
		draw_line(image, p2, p0, stroke_color);
	}
}

fn fill_rect(image: &mut Image, p0: Point2, width: i32, height: i32, color: Color) {
	let x0 = p0.x;
	let x1 = x0 + width;
	let y0 = p0.y;
	let y1 = y0 + height;
	for y in y0..=y1 {
		for x in x0..=x1 {
			put_pixel(image, x, y, color);
		}
	}
}

fn shade_triangle(image: &mut Image, p0: Point2, p1: Point2, p2: Point2, color: Color) {
	let mut p0 = p0;
	let mut p1 = p1;
	let mut p2 = p2;
	if p1.y < p0.y { swap(&mut p1, &mut p0)} 
	if p2.y < p0.y { swap(&mut p2, &mut p0)} 
	if p2.y < p1.y { swap(&mut p2, &mut p1)} 

	let h0 = p0.h;
	let h1 = p1.h;
	let h2 = p2.h;

	let mut x01 = utils::interpolate(p0.y, p0.x as f32, p1.y, p1.x as f32);
	let mut h01 = utils::interpolate(p0.y, h0, p1.y, h1);

	let x12 = utils::interpolate(p1.y, p1.x as f32, p2.y, p2.x as f32);
	let h12 = utils::interpolate(p1.y, h1, p2.y, h2);
	
	let x02 = utils::interpolate(p0.y, p0.x as f32, p2.y, p2.x as f32);
	let h02 = utils::interpolate(p0.y, h0, p2.y, h2);

	x01.pop(); // last element is repeated in x12
	let x012 = [x01, x12].concat();

	h01.pop(); // last element is repeated in h12
	let h012 = [h01, h12].concat();
	
	let x_left;
	let x_right;
	let h_left;
	let h_right;

	let mid = (x02.len() / 2) as f32;
	let mid = math::floor_f(mid) as usize;

	if x02[mid] < x012[mid] {
		x_left = &x02;
		h_left = &h02;
		x_right = &x012;
		h_right = &h012;
	} else {
		x_left = &x012;
		h_left = &h012;
		x_right = &x02;
		h_right = &h02;
	}

	let y0 = p0.y;
	let y2 = p2.y;
	for y in y0..=y2 {
		let y_idx = (y - y0) as usize;
		let xl = x_left[y_idx] as i32;
		let xr = x_right[y_idx] as i32;
		let h_segment = utils::interpolate(xl, h_left[y_idx], xr, h_right[y_idx]);
		for x in xl..=xr {
			let x_idx = (x - xl) as usize;
			let h = h_segment[x_idx];
			let shaded_color = utils::mul_color(&color, h);
			put_pixel(image, x, y, shaded_color)
		}
	}
	
}

fn render_triangle(image: &mut Image, triangle: &Triangle, projected: &Vec<Point2>) {
	let p0 = projected[triangle.v0 as usize];
	let p1 = projected[triangle.v1 as usize];
	let p2 = projected[triangle.v2 as usize];
	let stroke = Some(triangle.color);
	let fill = None;
	draw_triangle(image, p0, p1, p2, stroke, fill)
}

pub fn render_shape(image: &mut Image, cam: &Camera, verticies: &[Vertex], triangles: &[Triangle]) {
	let mut projected: Vec<Point2> = vec![];
	for v in verticies {
		projected.push(utils::project_vertex(cam, v));
	}

	for t in triangles {
		render_triangle(image, t, &projected);
	}
}

pub fn render_scene() {
	
}

fn put_pixel(image: &mut Image, x: i32, y: i32, color: Color) {
	let (width, height) = get_canvas_dimensions();

	let (x_mapped, y_mapped) = utils::map_to_pixels(x, y, width as i32, height as i32);
    image.set_pixel(x_mapped, y_mapped, color);
}