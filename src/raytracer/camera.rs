use super::{mat3::Mat3, vec3::Point};

#[derive(Default, Debug, Clone)]
pub struct Camera {
    pub position: Point,
    pub rotation: Mat3,
}

impl Camera {
    pub fn new(p: Point, r: Mat3) -> Camera {
        Camera {
            position: p,
            rotation: r,
        }
    }
}
