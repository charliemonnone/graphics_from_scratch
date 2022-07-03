use macroquad::prelude::Color;
use crate::{math, sphere::Sphere, light::{LightType, LightSource}, vec3::{self, Vec3, Point3}};

#[derive(Debug, Default)]
pub struct Scene {
	pub spheres: Vec<Sphere>,
	pub lights: Vec<LightSource>,
	pub background_color: Color
}

impl Scene {
	pub fn new(spheres: Vec<Sphere>, lights: Vec<LightSource>, bg: Color) -> Self {
		Self { spheres, lights, background_color: bg }
	}

	pub fn test_scene() -> Self {
		let spheres = vec![
			Sphere::new(Point3::new(0.0, 1.0, 3.0), 1.0, Color::from_rgba(255, 100, 120, 255)),
			Sphere::new(Point3::new(2.0, 0.0, 4.0), 1.0, Color::from_rgba(100, 150, 255, 255)),
			Sphere::new(Point3::new(-2.0, 0.0, 4.0), 1.0, Color::from_rgba(100, 255, 150, 255)),
			Sphere::new(Point3::new(0.0, 5001.0, 0.0), 5000.0, Color::from_rgba(255, 200, 0, 255)),
		];

		// let lights:Vec<Box<dyn Light>> = vec![
		//     Box::new(AmbientLight::new(0.2)),
		//     Box::new(PointLight::new(0.6, Vec3::new(2.0, 1.0, 0.0))),
		//     Box::new(DirectionalLight::new(0.2, Vec3::new(1.0, 4.0, 4.0)))
		// ];

		let lights = vec![
			LightSource::new(LightType::Ambient, 0.5, None, None),
			LightSource::new(LightType::Point, 0.4, Some(Vec3::new(2.0, 1.0, 0.0)), None),
			LightSource::new(LightType::Directional, 0.1, None, Some(Vec3::new(1.0, 4.0, 4.0)))
		];
		let bg = Color::from_rgba(255, 255, 255, 255);

		Scene::new(spheres, lights, bg)
	}

	pub fn trace_ray(&self, origin: &Point3, direction: &Point3, t_min: f32, t_max: f32) -> Color {
		let mut closest_t = math::INFINITY;
		let mut closest_sphere: Option<&Sphere> = None;
		let spheres = &self.spheres;
		for sphere in spheres {
			let (t1, t2) = self.intersect_ray_sphere(origin, direction, sphere);
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
			Some(sphere) => {
				let position = vec3::add(&origin, &vec3::scale(direction, closest_t)); // intersection
				let mut normal = vec3::sub(&position, &sphere.center);
				normal = vec3::scale(&normal, 1.0 / math::vec_length(&normal));
				let intensity = self.compute_lighting(position, normal);
				Color::new(sphere.color.r*intensity, sphere.color.g*intensity, sphere.color.b*intensity, 255.0)
			}
			// TODO: replace with reflection computations,
			// TODO: refactor point3 to be vec3, or make a camera struct w/position as vec3
			None => self.background_color
		}
	}

	fn intersect_ray_sphere(&self, origin: &Point3, direction: &Point3, sphere: &Sphere) -> (f32, f32) {
		let radius = sphere.radius;
		let oc = vec3::sub(&origin, &sphere.center);
	
		let a = vec3::dot(&direction, &direction);
		let b = 2.0 * vec3::dot(&oc, &direction);
		let c = vec3::dot(&oc, &oc) - radius*radius;
		let discriminant = b*b - 4.0*a*c;
	
		if discriminant < 0.0 {
			(math::INFINITY, math::INFINITY)
		} else {
			let t1 = (-b + math::sqrt(discriminant)) / (2.0*a);
			let t2 = (-b - math::sqrt(discriminant)) / (2.0*a);
	
			(t1, t2)
		}
	}

	fn compute_lighting(&self, position: Vec3<f32>, normal: Vec3<f32>) -> f32 {
		let mut intensity = 0.0;
		let mut l = None; // light direction
		let lights = &self.lights;
		for light in lights {
			match light.light_type {
				LightType::Ambient => {
					intensity += light.intensity
				},
				LightType::Point => {
					let p = light.position.expect("Point light without position");
					l = Some(vec3::sub(&p, &position));

				},
				LightType::Directional => {
					l = light.direction;
				}
			}

			if let Some(l) = l {
				let nl = vec3::dot(&normal, &l);
				if nl > 0.0 {
					intensity += light.intensity * nl / (math::vec_length(&normal) * math::vec_length(&l))
				}
			}
		}

		intensity
	}
}


