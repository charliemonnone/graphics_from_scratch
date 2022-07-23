use super::{data_types::{Instance, Model}, camera::Camera, vec3::Position, render, constants::*};
use macroquad::prelude::{Image};
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
	} else {
		return;
	}
}

fn cube_scene(image: &mut Image) {
	let camera = Camera::new(Position::default(), 1.0, 1.0, 1.0);

	let cube_model: Model = Model::from(&CUBE);
	let instances = vec![
		Instance::new(&cube_model, Position::new(-1.5, 0., 7.)),
		Instance::new(&cube_model, Position::new(1.25, -2., 7.5)),
	];


	render::render_scene(image, &camera, &instances)
}