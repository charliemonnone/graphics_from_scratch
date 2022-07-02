use crate::sphere::Sphere;

#[derive(Debug, Default)]
pub struct Scene {
	pub spheres: Vec<Sphere>
}

// trait SceneObj {
	// pub fn center(&self) -> &Point3 {}
// }

impl Scene {
	pub fn new(spheres: Vec<Sphere>) -> Self {
		Self { spheres }
	}
}