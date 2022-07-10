use crate::raytracer::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum LightType {
    Ambient,
    Point,
    Directional,
}

#[derive(Debug, Copy, Clone)]
pub struct LightSource {
    pub light_type: LightType,
    pub intensity: f32,
    pub position: Option<Vec3<f32>>,
    pub direction: Option<Vec3<f32>>,
}

impl LightSource {
    pub fn new(light_type: LightType, intensity: f32, position: Option<Vec3<f32>>, direction: Option<Vec3<f32>>) -> Self {
        LightSource {
            light_type,
            intensity,
            position,
            direction,
        }
    }
}
