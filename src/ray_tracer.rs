use macroquad::prelude::{Image, Color};

use crate::canvas::Canvas;
use crate::color::{Color24};
use crate::point::Point3;
use crate::sphere::Sphere;
use crate::scene::Scene;
use crate::math;

pub const VIEWPORT_SIZE: f32 = 1.0;
pub const PROJECTION_PLANE_Z: f32 = 1.0;
pub const CAMERA_POSITION:Point3 =  Point3 {x: 0.0, y: 0.0, z: 0.0};

pub const RED:   Color24 = Color24 {r: 255, g: 0, b: 0};
pub const BLUE:  Color24 = Color24 {r: 0, g: 0, b: 255};
pub const GREEN: Color24 = Color24 {r: 0, g: 255, b: 0};
pub const WHITE: Color24 = Color24 {r: 255, g: 255, b: 255};
pub const BLACK: Color24 = Color24 {r: 0, g: 0, b: 0};

const BACKGROUND_COLOR: Color24 = WHITE;

pub fn run(image: &mut Image, width: f32, height: f32) {
    let canvas = Canvas::new(width as i32, height as i32);
    let mut scene = Scene::default();
    let s1 = Sphere::new(Point3::new(0.0, 1.0, 3.0), 1.0, RED);
    let s2 = Sphere::new(Point3::new(2.0, 0.0, 4.0), 1.0, BLUE);
    let s3 = Sphere::new(Point3::new(-2.0, 0.0, 4.0), 1.0, GREEN);
    
    scene.spheres.push(s1);
    scene.spheres.push(s2);
    scene.spheres.push(s3);

    let canvas_width = canvas.width;
    let canvas_height = canvas.height;
    for y in -canvas_height/2..canvas_height/2  {
        for x in -canvas_width/2..canvas_width/2 { 

            let direction = canvas_to_viewport(x as f32, y as f32, canvas_width as f32, canvas_height as f32);
            let color = trace_ray(&scene, &CAMERA_POSITION, &direction, 1.0 , math::INFINITY);
            let x_mapped = (x + (canvas_width/2)) as u32;
            let y_mapped = (y + (canvas_height/2)) as u32;
            put_pixel(image, x_mapped, y_mapped, &color);


        }
    }
    
}

fn put_pixel(image: &mut Image, x: u32, y: u32, color: &Color24) {
    let rgba = Color::new(color.r as f32, color.g as f32, color.b as f32, 255.0);
    image.set_pixel(x, y, rgba);
}


// NOTE: probably should be a scene function
fn trace_ray(scene: &Scene, origin: &Point3, direction: &Point3, t_min: f32, t_max: f32) -> Color24 {
    let mut closest_t = math::INFINITY;
    let mut closest_sphere: Option<&Sphere> = None;
    
    for sphere in &scene.spheres {
        let (t1, t2) = intersect_ray_sphere(origin, direction, sphere);
        let range = t_min..closest_t;
        if range.contains(&t1) && t1 < t_max {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }
        if range.contains(&t2) && t2 < t_max {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }

    }

    match closest_sphere {
        Some(sphere) => sphere.color, // NOTE: allocates new Color24 
        None => BACKGROUND_COLOR
    }
}

fn intersect_ray_sphere(origin: &Point3, direction: &Point3, sphere: &Sphere) -> (f32, f32) {
    let radius = sphere.radius;
    let oc = math::sub(origin, &sphere.center);

    let a = math::dot(direction, direction);
    let b = 2.0 * math::dot(&oc, direction);
    let c = math::dot(&oc,&oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        (math::INFINITY, math::INFINITY)
    } else {
        let t1 = (-b + math::sqrt(discriminant)) / (2.0*a);
        let t2 = (-b - math::sqrt(discriminant)) / (2.0*a);

        (t1, t2)
    }
}

fn canvas_to_viewport(x: f32, y: f32, canvas_width: f32, canvas_height: f32) -> Point3 {
    Point3::new(
        x * VIEWPORT_SIZE / canvas_width, 
        y * VIEWPORT_SIZE / canvas_height, 
        PROJECTION_PLANE_Z
    )
}