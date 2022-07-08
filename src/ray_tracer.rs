use crate::{color::Color, math, scene::Scene, vec3::Point};
use macroquad::prelude::Image;

pub const VIEWPORT_SIZE: f32 = 1.0;
pub const PROJECTION_PLANE_Z: f32 = 0.50;
pub const CAMERA_POSITION: Point = Point::new(0.0, 0.0, 0.0);

pub fn run(image: &mut Image, width: f32, height: f32) {
    let scene = Scene::test_scene();
    let image_width = width;
    let image_height = width;
    let width = width as i32;
    let height = height as i32;
    let recursion_limit = 3;

    for y in -height / 2..height / 2 {
        for x in -width / 2..width / 2 {
            let direction = image_to_viewport(x as f32, y as f32, image_width, image_height);
            let color = scene.trace_ray(&CAMERA_POSITION, &direction, 1.0, math::INFINITY, recursion_limit);
            let x_mapped = (x + (width / 2)) as u32;
            let y_mapped = (y + (height / 2)) as u32;

            put_pixel(image, x_mapped, y_mapped, color);
        }
    }
}

fn put_pixel(image: &mut Image, x: u32, y: u32, color: Color) {
    image.set_pixel(x, y, color);
}

fn image_to_viewport(x: f32, y: f32, image_width: f32, image_height: f32) -> Point {
    Point::new(
        x * VIEWPORT_SIZE / image_width,
        y * VIEWPORT_SIZE / image_height,
        PROJECTION_PLANE_Z,
    )
}
