use crate::{color::{Color, self}, math, scene::{Scene}, vec3::{Point}, camera::{Camera}, mat3::Mat3};
use macroquad::prelude::Image;

pub const VIEWPORT_SIZE: f32 = 1.0;
pub const PROJECTION_PLANE_Z: f32 = 0.50;

pub fn run(image: &mut Image, width: f32, height: f32) {
    let mut scene = Scene::test_scene();
    let image_width = width;
    let image_height = width;
    let width = width as i32;
    let height = height as i32;
    let recursion_limit = 3;

    let camera = Camera::new(
        Point::new(3.0, 0.0, 1.0), 
        Mat3::new(vec![0.7071, 0.0, -0.7071, 0.0, 1.0, 0.0, 0.7071, 0.0, 0.7071]) // 3x3 matrix flattened to 1d array
    );
    let subsampling = 1;
    let mut color = color::BLACK;

    // TODO: split scene into quadrants, multithread tracing 
    // use rayon, or create 4 smaller images and then stitch them together in a texture after ray tracing finishes
    // TODO: introduce notion of "last hit sphere" to each thread, 
    // if it's the same as last iteration use cached dot(camera.pos, sphere.center)
    for y in -height / 2..height / 2 {
        for x in -width / 2..width / 2 {
            let direction = image_to_viewport(x as f32, y as f32, image_width, image_height);
            let direction = camera.rotation.mul_vec3(&direction);
            if x % subsampling == 0 {
                color = scene.trace_ray(&camera.position, &direction, 1.0, math::INFINITY, recursion_limit);

            }

            let (x_mapped, y_mapped) = map_to_pixels(x, y, width, height);
            put_pixel(image, x_mapped, y_mapped, color);
        }
    }
}

fn put_pixel(image: &mut Image, x: u32, y: u32, color: Color) {

    image.set_pixel(x, y, color);
}

fn map_to_pixels(x: i32, y: i32, width: i32, height: i32 ) -> (u32, u32) {
    let x_mapped = (x + (width / 2)) as u32;
    let y_mapped = (y + (height / 2)) as u32;
    (x_mapped, y_mapped)
}

fn image_to_viewport(x: f32, y: f32, image_width: f32, image_height: f32) -> Point {
    Point::new(
        x * VIEWPORT_SIZE / image_width,
        y * VIEWPORT_SIZE / image_height,
        PROJECTION_PLANE_Z,
    )
}
