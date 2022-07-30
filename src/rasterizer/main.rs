use super::{data_types::{Instance, Model, Vertex3, Cube}, camera::Camera, render, utils};
use macroquad::prelude::{Image};
use once_cell::sync::Lazy;
use std::{sync::{RwLock}};

#[derive(Debug, Default)]
pub struct GlobalCtx {
	width: f32,
	height: f32,
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

	let init = init_global_ctx(width, height);
	if init {
		cube_scene(image);
	} else {
		return;
	}
}

fn cube_scene(image: &mut Image) {
	let camera = Camera::new(Vertex3::new(-3., 1., 2.), utils::make_rotation_mat(-30.), 0.75, 1.0, 1.0);
	let base_cube: Cube = Cube::debug_cube();
	let cube_model: Model = Model::from(&base_cube);

	let instances = vec![
		Instance::new(&cube_model, Vertex3::new(-1.5, 0., 7.), None, Some(0.75)),
		Instance::new(&cube_model, Vertex3::new(1.25, 2.5, 7.5), Some(utils::make_rotation_mat(195.)), None),
	];

	render::render_scene(image, &camera, &instances)
}