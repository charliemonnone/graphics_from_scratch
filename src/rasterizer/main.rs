use std::{mem::swap};
use super::{math, point2::Point2, vertex::Vertex, camera::Camera, vec3::Position};
use macroquad::prelude::{Image, Color, BLACK, RED, screen_width, screen_height, BLUE, YELLOW, ORANGE, PURPLE, GREEN};

pub fn run(image: &mut Image, width: f32, height: f32) {
	test_scene(image, width, height);
	cube_scene(image, width, height);
}

fn draw_line(image: &mut Image, p0: Point2, p1: Point2, color: Color) {
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
		let ys = interpolate(x0, y0 as f32, x1, y1 as f32);
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
		let xs = interpolate(y0, x0 as f32, y1, x1 as f32);

		for y in y0..=y1 {
			let index: usize = (y - y0) as usize;
			let x = xs[index];
			put_pixel(image, x as i32, y, color);
		}
	}
}

/// Given upper left point p0, a width and height, draw a rectangle
fn draw_rect(image: &mut Image, p0: Point2, width: i32, height: i32, color: Color, fill: Option<Color>) {
	
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

fn draw_triangle(image: &mut Image, p0: Point2, p1: Point2, p2: Point2, stroke: Option<Color>, fill: Option<Color>) {
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

	let mut x01 = interpolate(p0.y, p0.x as f32, p1.y, p1.x as f32);
	let mut h01 = interpolate(p0.y, h0, p1.y, h1);

	let x12 = interpolate(p1.y, p1.x as f32, p2.y, p2.x as f32);
	let h12 = interpolate(p1.y, h1, p2.y, h2);
	
	let x02 = interpolate(p0.y, p0.x as f32, p2.y, p2.x as f32);
	let h02 = interpolate(p0.y, h0, p2.y, h2);

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
		let h_segment = interpolate(xl, h_left[y_idx], xr, h_right[y_idx]);
		for x in xl..=xr {
			let x_idx = (x - xl) as usize;
			let h = h_segment[x_idx];
			let shaded_color = mul_color(&color, h);
			put_pixel(image, x, y, shaded_color)
		}
	}
	
}

fn interpolate(i0: i32, d0: f32, i1: i32, d1: f32) -> Vec<f32> {
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

fn mul_color(color: &Color, h: f32) -> Color {
	Color::new(color.r * h, color.g * h, color.b * h, color.a)
}

fn put_pixel(image: &mut Image, x: i32, y: i32, color: Color) {
	let (x_mapped, y_mapped) = map_to_pixels(x, y, screen_width() as i32, screen_height() as i32);
    image.set_pixel(x_mapped, y_mapped, color);
}

fn map_to_pixels(x: i32, y: i32, width: i32, height: i32 ) -> (u32, u32) {
    let x_mapped = (x + (width /2)) as u32;
    let y_mapped = (y + (height /2)) as u32;
    (x_mapped, y_mapped)
}

fn viewport_to_canvas(cam: &Camera, x: f32, y: f32) -> Point2 {
	Point2::new((x * screen_width() / cam.view_width) as i32 , (y * screen_height() / cam.view_height) as i32, 1.0)
}

fn project_vertex(cam: &Camera, v: &Vertex) -> Point2 {
	viewport_to_canvas(cam, v.x * cam.viewport_dist / v.z, v.y * cam.viewport_dist / v.z)
}

fn test_scene(image: &mut Image, width: f32, height: f32) {
	
	let camera = Camera::new(Position::default(), 1.0, 1.0, 1.0);
	
	let p0 = Point2::new(0, 0, 1.0);
	let p1 = Point2::new(100, 100, 1.0);
	let p2 = Point2::new(200, 0, 1.0);

	let tri_p0 = Point2::new(-200, 250, 0.3);
	let tri_p1 = Point2::new(200, -50, 0.1);
	let tri_p2 = Point2::new(20, -250, 1.0);
	draw_rect(image, p0, 200, 100, RED, Some(RED));
	draw_rect(image, p1, 200, 100, ORANGE, Some(ORANGE));
	draw_rect(image, p0, 100, 200, BLUE, Some(BLUE));
	draw_rect(image, p2, 100, 100, PURPLE, Some(PURPLE));
	draw_triangle(image, tri_p0, tri_p1, tri_p2, None, Some(GREEN));
}

fn cube_scene(image: &mut Image, width: f32, height: f32) {
	let camera = Camera::new(Position::default(), 1.0, 1.0, 1.0);

	let v_af = Vertex::new(-2.0, 0.5, 5.0);
	let v_bf = Vertex::new(-2.0, -0.5, 5.0);
	let v_cf = Vertex::new(-1.0, -0.5, 5.0);
	let v_df = Vertex::new(-1.0, 0.5, 5.0);

	let v_ab = Vertex::new(-2.0, 0.5, 6.0);
	let v_bb = Vertex::new(-2.0, -0.5, 6.0);
	let v_cb = Vertex::new(-1.0, -0.5, 6.0);
	let v_db = Vertex::new(-1.0, 0.5, 6.0);

	// front face
	draw_line(image, project_vertex(&camera, &v_af), project_vertex(&camera, &v_bf), BLUE);
	draw_line(image, project_vertex(&camera, &v_bf), project_vertex(&camera, &v_cf), BLUE);
	draw_line(image, project_vertex(&camera, &v_cf), project_vertex(&camera, &v_df), BLUE);
	draw_line(image, project_vertex(&camera, &v_df), project_vertex(&camera, &v_af), BLUE);

	// back face
	draw_line(image, project_vertex(&camera, &v_ab), project_vertex(&camera, &v_bb), RED);
	draw_line(image, project_vertex(&camera, &v_bb), project_vertex(&camera, &v_cb), RED);
	draw_line(image, project_vertex(&camera, &v_cb), project_vertex(&camera, &v_db), RED);
	draw_line(image, project_vertex(&camera, &v_db), project_vertex(&camera, &v_ab), RED);

	// front to back face
	draw_line(image, project_vertex(&camera, &v_af), project_vertex(&camera, &v_ab), GREEN);
	draw_line(image, project_vertex(&camera, &v_bf), project_vertex(&camera, &v_bb), GREEN);
	draw_line(image, project_vertex(&camera, &v_cf), project_vertex(&camera, &v_cb), GREEN);
	draw_line(image, project_vertex(&camera, &v_df), project_vertex(&camera, &v_db), GREEN);


}