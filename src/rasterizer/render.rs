use std::mem::swap;
use super::{data_types::Point, utils, main::get_canvas_dimensions, data_types::{Instance}, data_types::{Triangle, Model, Mat4x4, Vertex4}, camera::Camera};
use macroquad::{texture::Image, color::Color};

pub fn draw_line(image: &mut Image, p0: Point, p1: Point, color: Color) {
	let mut p0 = p0;
	let mut p1 = p1;

	let mut x0 = p0.x;
	let mut y0 = p0.y;
	let mut x1 = p1.x;
	let mut y1 = p1.y;

	let dx = x1 - x0;
	let dy = y1 - y0;
	
	if utils::math::abs(dx) > utils::math::abs(dy) {
		// horizontalish
		if x0 > x1 { 
			swap(&mut p0, &mut p1); 
			x0 = p0.x;
			y0 = p0.y;
			x1 = p1.x;
			y1 = p1.y;
		}
		let ys = utils::interpolate(x0, y0 as f32, x1, y1 as f32);
		let start = x0 as i32;
		let end = x1 as i32;
		for x in start..=end {
			let index: usize = (x as f32 - x0) as usize;
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
		let start = y0 as i32;
		let end = y1 as i32;
		for y in start..=end {
			let index: usize = (y as f32 - y0) as usize;
			let x = xs[index];
			put_pixel(image, x as i32, y, color);
		}
	}
}

fn draw_triangle(image: &mut Image, p0: Point, p1: Point, p2: Point, stroke: Option<Color>, fill: Option<Color>) {
	if let Some(fill_color) = fill {
		shade_triangle(image, p0, p1, p2, fill_color);
	}

	if let Some(stroke_color) = stroke {
		draw_line(image, p0, p1, stroke_color);
		draw_line(image, p1, p2, stroke_color);
		draw_line(image, p2, p0, stroke_color);
	}
}

fn shade_triangle(image: &mut Image, p0: Point, p1: Point, p2: Point, color: Color) {
	let mut p0 = p0;
	let mut p1 = p1;
	let mut p2 = p2;
	if p1.y < p0.y { swap(&mut p1, &mut p0)} 
	if p2.y < p0.y { swap(&mut p2, &mut p0)} 
	if p2.y < p1.y { swap(&mut p2, &mut p1)} 

	let h0 = p0.z;
	let h1 = p1.z;
	let h2 = p2.z;

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
	let mid = utils::math::floor_f(mid) as usize;

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

	let start = p0.y as i32;
	let end = p2.y as i32;

	for y in start..=end {
		let y_idx = (y - start) as usize;
		let xl = x_left[y_idx] as i32;
		let xr = x_right[y_idx] as i32;
		let h_segment = utils::interpolate(xl as f32, h_left[y_idx], xr as f32, h_right[y_idx]);
		for x in xl..=xr {
			let x_idx = (x - xl) as usize;
			let h = h_segment[x_idx];
			let shaded_color = utils::mul_color(&color, h);
			put_pixel(image, x, y, shaded_color)
		}
	}
	
}

fn render_triangle(image: &mut Image, triangle: &Triangle, projected: &Vec<Point>) {
	let p0 = projected[triangle.v0 as usize];
	let p1 = projected[triangle.v1 as usize];
	let p2 = projected[triangle.v2 as usize];
	let stroke = Some(triangle.color);
	let fill = None;
	draw_triangle(image, p0, p1, p2, stroke, fill)
}

pub fn render_scene(image: &mut Image, cam: &Camera, instances: &Vec<Instance>) {
	let camera_mat = utils::mul_mm(cam.orientation.transpose(), utils::make_translation_mat(cam.pos * -1.));

	for i in instances {
		let transform = utils::mul_mm(camera_mat, i.transform);
		render_model(image, cam, i.model, transform);
	}
}

pub fn render_model(image: &mut Image, cam: &Camera, model: &Model, transform: Mat4x4) {
	let mut projected: Vec<Point> = vec![];

	for v in &model.verticies {
		let v = Vertex4::new(v.x, v.y, v.z, 1.);
		let v4 = utils::mul_mv(transform, v);
		projected.push(utils::project_vertex(cam, v4));
	}

	for t in &model.triangles {
		render_triangle(image, t, &projected)
	}
}

fn put_pixel(image: &mut Image, x: i32, y: i32, color: Color) {
	let (width, height) = get_canvas_dimensions();
	let (x_mapped, y_mapped) = utils::map_to_pixels(x, y, width as i32, height as i32);

	if x_mapped >= width as u32 || y_mapped >= height as u32 { return; }

	image.set_pixel(x_mapped, y_mapped, color);
}