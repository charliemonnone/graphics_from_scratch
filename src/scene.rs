use crate::{
    light::{LightSource, LightType},
    math::{self, vec_length},
    sphere::Sphere,
    vec3::{Point, Vec3, dot, neg},
};
use macroquad::prelude::{Color, RED, BLUE, GREEN, YELLOW, BLACK};

#[derive(Debug, Default)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<LightSource>,
    pub background_color: Color,
}

const EPISLON: f32 = 0.001;

impl Scene {
    pub fn new(spheres: Vec<Sphere>, lights: Vec<LightSource>, bg: Color) -> Self {
        Self {
            spheres,
            lights,
            background_color: bg,
        }
    }
    //  Color::from_rgba(255, 100, 120, 255), redish
    //  Color::from_rgba(100, 150, 255, 255), bluish
    //  Color::from_rgba(100, 255, 150, 255), greenish
    //  Color::from_rgba(255, 200, 0, 255), yellowish
    pub fn test_scene() -> Self {
        let spheres = vec![
            Sphere::new(Point::new(0.0, 1.0, 3.0), 1.0, Color::from_rgba(255, 0, 0, 255), 500.0, 0.2),
            Sphere::new(Point::new(2.0, 0.0, 4.0), 1.0, Color::from_rgba(0, 0, 255, 255), 500.0, 0.3),
            Sphere::new(Point::new(-2.0, 0.0, 4.0), 1.0, Color::from_rgba(0, 255, 0, 255), 10.0, 0.4),
            Sphere::new(Point::new(0.0, 5001.0, 0.0), 5000.0, Color::from_rgba(255, 255, 0, 255), 1000.0, 0.5),
        ];

        let lights = vec![
            LightSource::new(LightType::Ambient, 0.2, None, None),
            LightSource::new(LightType::Point, 0.6, Some(Vec3::new(2.0, -1.0, 0.0)), None),
            LightSource::new(LightType::Directional, 0.2, None, Some(Vec3::new(1.0, -4.0, 4.0))),
        ];

        let bg = BLACK;

        Scene::new(spheres, lights, bg)
    }

    pub fn trace_ray(&self, origin: &Point, direction: &Point, t_min: f32, t_max: f32, recursion_depth: i32) -> Color {

        let (closest_sphere, closest_t) = self.closest_intersection(origin, direction, t_min, t_max);

        match closest_sphere {
            Some(sphere) => {
                let position = origin + &(direction * closest_t); // intersection
                let mut normal = position - sphere.center;
                normal = normal * (1.0 / math::vec_length(&normal));                 
				let intensity = self.compute_lighting(&position, &normal, &neg(direction), sphere.specular);
                let mut local_color = mul_color(&sphere.color, intensity);
                let reflectivity = sphere.reflective;
                if recursion_depth <= 0 || reflectivity <= 0.0 {
                    return local_color
                }

                let ray = reflect_ray(&-direction, &normal);
                let mut reflected_color = self.trace_ray(&position, &ray, t_min, t_max, recursion_depth - 1);
                reflected_color = mul_color(&reflected_color, reflectivity);
                local_color = mul_color(&local_color, 1.0 - reflectivity);
                
                Color::new(
                    local_color.r + reflected_color.r, 
                    local_color.g + reflected_color.g, 
                    local_color.b + reflected_color.b, 
                    255.0
                )
            }
            None => self.background_color,
        }
    }

    fn closest_intersection(&self, origin: &Vec3<f32>, direction: &Vec3<f32>, t_min: f32, t_max: f32) -> (Option<&Sphere>, f32) {
        let mut closest_t = math::INFINITY;
        let mut closest_sphere: Option<&Sphere> = None; 
        let spheres = &self.spheres;

        for sphere in spheres {
            let (t1, t2) = self.intersect_ray_sphere(origin, direction, sphere);
            let range = t_min..t_max;
            if range.contains(&t1) && t1 < closest_t {
                closest_t = t1;
                closest_sphere = Some(sphere);
            }
            if range.contains(&t2) && t2 < closest_t {
                closest_t = t2;
                closest_sphere = Some(sphere);
            }
        }
        (closest_sphere, closest_t)
    }

    fn intersect_ray_sphere(&self, origin: &Point, direction: &Point, sphere: &Sphere) -> (f32, f32) {
        let radius = sphere.radius;
        let oc = origin - &sphere.center;

        let a = dot(&direction, &direction);
        let b = 2.0 * dot(&oc, &direction);
        let c = dot(&oc, &oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            (math::INFINITY, math::INFINITY)
        } else {
            let t1 = (-b + math::sqrt_f32(discriminant)) / (2.0 * a);
            let t2 = (-b - math::sqrt_f32(discriminant)) / (2.0 * a);

            (t1, t2)
        }
    }

    fn compute_lighting(&self, position: &Vec3<f32>, normal: &Vec3<f32>, direction: &Vec3<f32>, specularity: f32) -> f32 {
        let mut intensity = 0.0;
        let mut l = None; 
        let mut t_max = math::INFINITY;
        let lights = &self.lights;

        for light in lights {
            match light.light_type {
                LightType::Ambient => intensity += light.intensity,
                LightType::Point => {
                    let p = light.position.expect("Point light without position");
                    l = Some(&p - position);
                    t_max = 1.0;
                }
                LightType::Directional => {
                    l = light.direction;
                }
            }
            if let Some(l) = l {

                // Shadow Check
                let (shadow_sphere, _shadow_t) = self.closest_intersection(position, &l, EPISLON, t_max);
                if shadow_sphere.is_some() {
                    continue;
                }

				// Diffuse
                let nl = dot(&normal, &l);
                if nl > 0.0 {
                    intensity +=
                        light.intensity * nl / (math::vec_length(&normal) * math::vec_length(&l))
                }

				// Specularity
				if specularity > -1.0 {
                    let r = reflect_ray(&l, &normal);
					let rv = dot(&r, &direction);
					if rv > 0.0 {
						let r_len = vec_length(&r);
						let cam_dir_len = vec_length(&direction);
						intensity += light.intensity * math::pow(rv / (r_len * cam_dir_len), specularity);
					}
				}
            }

        }

        intensity
    }

}

/// reflect ray r with respect to the normal of the surface
fn reflect_ray(r: &Vec3<f32>, n: &Vec3<f32>) -> Vec3<f32> {
    &((n * 2.0) * dot(n, r)) - r
}

fn mul_color(color: &Color, n: f32) -> Color {
    Color::new(
        color.r * n,
        color.g * n,
        color.b * n,
        1.0
    )
}
