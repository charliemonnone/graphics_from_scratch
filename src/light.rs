use crate::vec3::*;

pub trait Light {

}

#[derive(Debug, Default, Copy, Clone)]
pub struct AmbientLight {
	intensity: f32
}

impl Light for AmbientLight {

}

impl AmbientLight {
	pub fn new(intensity: f32) -> Self {
		AmbientLight { intensity }
	}
}

#[derive(Debug, Default, Copy, Clone)]
pub struct PointLight {
	intensity: f32,
	position: Vec3<f32>
}

impl Light for PointLight {
	
}

impl PointLight {
	pub fn new(intensity: f32, position: Vec3<f32>) -> Self {
		PointLight { intensity, position }
	}
}

#[derive(Debug, Default, Copy, Clone)]
pub struct DirectionalLight {
	intensity: f32,
	direction: Vec3<f32>
}

impl Light for DirectionalLight {
	
}

impl DirectionalLight {
	pub fn new(intensity: f32, direction: Vec3<f32>) -> Self {
		DirectionalLight { intensity, direction }
	}
}

#[derive(Debug, Copy, Clone)]
pub enum LightType {
	Ambient, 
	Point, 
	Directional
} 

#[derive(Debug, Copy, Clone)]
pub struct LightSource {
	pub light_type: LightType,
	pub intensity: f32,
	pub position: Option<Vec3<f32>>,
	pub direction: Option<Vec3<f32>>,
}

impl LightSource {
	pub fn new(
		light_type: LightType, 
		intensity: f32, 
		position: Option<Vec3<f32>>, 
		direction: Option<Vec3<f32>>) -> Self {
			LightSource {
				light_type,
				intensity, 
				position, 
				direction
			}
		}
}