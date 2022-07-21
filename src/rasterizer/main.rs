use super::{point2::Point2, vertex::Vertex, camera::Camera, vec3::Position, render, utils, shapes};
use macroquad::prelude::{Image, BLACK, RED, BLUE, YELLOW, ORANGE, PURPLE, GREEN, WHITE, Color};
use once_cell::sync::Lazy;
use std::{sync::{RwLock}};

#[derive(Debug, Default)]
pub struct GlobalCtx {
	width: f32,
	height: f32
}

static GLOBAL_CTX: Lazy<RwLock<GlobalCtx>> = Lazy::new(|| RwLock::new(GlobalCtx::default()));

pub fn get_canvas_dimensions() -> (f32, f32) {
	let g = GLOBAL_CTX.try_read().unwrap();
	(g.width, g.height)
}

pub fn init_global_ctx(width: f32, height: f32) -> bool {
	let g = GLOBAL_CTX.try_write();
	match g {
		Ok(mut global_ctx) => {
			global_ctx.width = width;
			global_ctx.height = height;
			return true;
		
		},
		Err(_) => {
			println!("Couldn't write to global ctx");
			return false;
		},
	}
}

pub fn run(image: &mut Image, width: f32, height: f32) {

	let init = init_global_ctx( width, height);
	if init {
		cube_scene(image);
		// test_scene(image);

	} else {
		return;
	}
}

fn test_scene(image: &mut Image) {
	
	let camera = Camera::new(Position::default(), 1.0, 1.0, 1.0);
	
	let p0 = Point2::new(0, 0, 1.0);
	let p1 = Point2::new(100, 100, 1.0);
	let p2 = Point2::new(200, 0, 1.0);

	let tri_p0 = Point2::new(-200, 250, 0.3);
	let tri_p1 = Point2::new(200, -50, 0.1);
	let tri_p2 = Point2::new(20, -250, 1.0);
	render::draw_rect(image, p0, 200, 100, RED, Some(RED));
	render::draw_rect(image, p1, 200, 100, ORANGE, Some(ORANGE));
	render::draw_rect(image, p0, 100, 200, BLUE, Some(BLUE));
	render::draw_rect(image, p2, 100, 100, PURPLE, Some(PURPLE));
	render::draw_triangle(image, tri_p0, tri_p1, tri_p2, None, Some(GREEN));
}

fn cube_scene(image: &mut Image) {
	let camera = Camera::new(Position::default(), 1.0, 1.0, 1.0);
	let mut cube = shapes::CUBE.clone();
	let translation = vec![-1.5, 0., 7.];
	// translate cube verts to not be inside camera
	for v in &mut cube.verticies {
		v.x += translation[0];
		v.y += translation[1];
		v.z += translation[2];
	}
	render::render_shape(image, &camera, &cube.verticies, &cube.triangles);
}