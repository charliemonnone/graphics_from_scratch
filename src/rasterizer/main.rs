use super::{
    camera::Camera,
    utils::math,
    data_types::{Cube, Instance, Model, Vertex3, Vertex4},
    render, utils,
};
use macroquad::prelude::Image;
use once_cell::sync::Lazy;
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct GlobalCtx {
    width: f32,
    height: f32,
    view_width: f32,
    view_height: f32,
    viewport_dist: f32
}

static GLOBAL_CTX: Lazy<RwLock<GlobalCtx>> = Lazy::new(|| RwLock::new(GlobalCtx::default()));

pub fn get_canvas_dimensions() -> (f32, f32) {
    let g = GLOBAL_CTX.try_read().unwrap();
    (g.width, g.height)
}

pub fn get_view_dimensions() -> (f32, f32) {
    let g = GLOBAL_CTX.try_read().unwrap();
    (g.view_width, g.view_height)
}

pub fn get_viewport_dist() -> f32 {
    let g = GLOBAL_CTX.try_read().unwrap();
    g.viewport_dist
}

pub fn init_global_ctx(width: f32, height: f32) -> bool {
    let g = GLOBAL_CTX.try_write();
    match g {
        Ok(mut global_ctx) => {
            global_ctx.width = width;
            global_ctx.height = height;
            global_ctx.view_width = 1.0;
            global_ctx.view_height = 1.0;
            global_ctx.viewport_dist = 0.75;
            return true;
        }
        Err(_) => {
            println!("Couldn't write to global ctx");
            return false;
        }
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
    let camera = Camera::new(
        Vertex3::new(-3., 1., 2.),
        utils::make_rotation_mat(-30.),
    );
    let base_cube: Cube = Cube::debug_cube();
    let cube_model: Model = Model::from((&base_cube, Vertex4::new(0., 0., 0., 1.), math::sqrt_f(3.)));

    let instances = vec![
        Instance::new(&cube_model, Vertex3::new(-1.5, 0., 7.), None, Some(0.75)),
        Instance::new(
            &cube_model,
            Vertex3::new(1.25, 2.5, 7.5),
            Some(utils::make_rotation_mat(195.)),
            None,
        ),
        Instance::new(
            &cube_model, 
            Vertex3::new(0., 0., -10.), 
            Some(utils::make_rotation_mat(195.)),
            None
        )
    ];

    render::render_scene(image, &camera, &instances)
}
