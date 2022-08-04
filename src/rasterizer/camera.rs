use super::{data_types::{Mat4x4, Vertex3, Plane}, utils::math};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub pos: Vertex3,
    pub orientation: Mat4x4,
    pub clipping_planes: [Plane; 5]
}

fn clipping_planes(s2: f32) -> [Plane; 5] {
    [
        Plane::new(Vertex3::new(0., 0., 1.), -1.), // near
        Plane::new(Vertex3::new(s2, 0., s2), 0.),   // left
        Plane::new(Vertex3::new(-s2, 0., s2), 0.),  // right
        Plane::new(Vertex3::new(0., -s2, s2), 0.),  // top
        Plane::new(Vertex3::new(0., s2, s2), 0.),   // bottom
    ]
}

impl Camera {
    pub fn new(pos: Vertex3, orientation: Mat4x4) -> Self {
        let s2 = math::sqrt_f(2.);
        Self {
            pos,
            orientation,
            clipping_planes: clipping_planes(s2)
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let s2 = math::sqrt_f(2.);
        Self {
            pos: Vertex3::default(),
            orientation: Mat4x4::default(),
            clipping_planes: clipping_planes(s2)

        }
    }
}
