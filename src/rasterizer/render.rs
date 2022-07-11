use std::{mem::swap};
use crate::rasterizer::math;
use macroquad::prelude::{Image, Color, BLACK, RED, screen_width, screen_height, BLUE, YELLOW, ORANGE, PURPLE, GREEN};

#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
	x: i32,
	y: i32
}

impl Point {
	pub fn centered() -> Self {
		Self {
			x: (screen_width() / 2.) as i32,
			y: (screen_height() / 2.) as i32
		}

	}

	pub fn add(self, x: i32, y: i32) -> Self {
		Self::new(self.x + x, self.y + y)
	}
	
	pub fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

}

pub fn run(image: &mut Image, width: f32, height: f32) {
	let center_x = (width / 2.) as i32;
	let center_y = (height / 2.) as i32;
	let p0 = Point::centered();
	let mut p1 = Point::centered();
	let mut p2 = Point::centered();
	p1.x += 100;
	p1.y += 100;

	p2.x -= 100;
	p2.y -= 200;

	let tri_p0 = Point::centered().add(-200, 250);
	let tri_p1 = Point::centered().add(200, -50);
	let tri_p2 = Point::centered().add(20, -250);

	draw_rect(image, p0, 200, 100, RED, Some(RED));
	draw_rect(image, p1, 200, 100, ORANGE, Some(ORANGE));
	draw_rect(image, p0, 100, 200, BLUE, Some(BLUE));
	draw_triangle(image, tri_p0, tri_p1, tri_p2, GREEN, Some(PURPLE));
}

fn draw_line(image: &mut Image, p0: Point, p1: Point, color: Color) {
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
			put_pixel(image, x as u32, y as u32, color);
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
			put_pixel(image, x as u32, y as u32, color);
		}
	}
}

/// Given upper left point p0, a width and height, draw a rectangle
fn draw_rect(image: &mut Image, p0: Point, width: i32, height: i32, color: Color, fill: Option<Color>) {
	
	let p1 = Point::new(p0.x + width, p0.y); 
	let p2 = Point::new(p1.x, p0.y + height); 
	let p3 = Point::new(p0.x, p2.y); 
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

fn draw_triangle(image: &mut Image, p0: Point, p1: Point, p2: Point, color: Color, fill: Option<Color>) {
	draw_line(image, p0, p1, color);
	draw_line(image, p1, p2, color);
	draw_line(image, p2, p0, color);

	if let Some(fill_color) = fill {
		fill_triangle(image, p0, p1, p2, fill_color);
	}
}

fn fill_rect(image: &mut Image, p0: Point, width: i32, height: i32, color: Color) {
	let x0 = p0.x;
	let x1 = x0 + width;
	let y0 = p0.y;
	let y1 = y0 + height;
	for y in y0..=y1 {
		for x in x0..=x1 {
			put_pixel(image, x as u32, y as u32, color);
		}
	}
}

fn fill_triangle(image: &mut Image, p0: Point, p1: Point, p2: Point, color: Color) {
	let mut p0 = p0;
	let mut p1 = p1;
	let mut p2 = p2;
	if p1.y < p0.y { swap(&mut p1, &mut p0)} 
	if p2.y < p0.y { swap(&mut p2, &mut p0)} 
	if p2.y < p1.y { swap(&mut p2, &mut p1)} 

	let mut x01 = interpolate(p0.y, p0.x as f32, p1.y, p1.x as f32);
	let mut x12 = interpolate(p1.y, p1.x as f32, p2.y, p2.x as f32);
	let x02 = interpolate(p0.y, p0.x as f32, p2.y, p2.x as f32);

	x01.pop(); // last element is repeated in x12
	x01.append(&mut x12);
	let x012 = x01;
	
	println!("{} {}", x02.len(), x012.len());
	let x_left;
	let x_right;
	let mid = (x02.len() / 2) as f32;
	let mid = math::floor_f(mid) as usize;
	if x02[mid] < x012[mid] {
		x_left = &x02;
		x_right = &x012;
	} else {
		x_left = &x012;
		x_right = &x02;
	}
	let y0 = p0.y;
	let y2 = p2.y;
	for y in y0..=y2 {
		let xl = x_left[(y - y0) as usize] as i32;
		let xr = x_right[(y - y0) as usize] as i32;
		for x in xl..=xr {
			put_pixel(image, x as u32, y as u32, color)
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

fn put_pixel(image: &mut Image, x: u32, y: u32, color: Color) {
    image.set_pixel(x, y, color);
}