use macroquad::prelude::Color;
use super::{color::*, utils};

pub type Point		= glam::Vec3;
pub type Vertex3 	= glam::Vec3;
pub type Vertex4 	= glam::Vec4;
pub type Vec4 		= glam::Vec4;
pub type Mat4x4 	= glam::Mat4;
pub const IDEN_4X4: Mat4x4 = glam::const_mat4!(
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0]
);

#[derive(Debug, Default, Copy, Clone)]
pub struct Triangle {
	pub v0: u32,
	pub v1: u32,
	pub v2: u32,
	pub color: Color
}

impl Triangle {
	pub const fn new(v0: u32, v1: u32, v2: u32, color: Color) -> Self {
		Self { v0, v1, v2, color }
	}
}

#[derive(Debug, Default, Clone)]
pub struct Cube {
	pub triangles: [Triangle; 12],
	pub verticies: [Vertex3; 8]
}

impl Cube {
	pub fn debug_cube() -> Self {
		Self { 
			triangles: [
				Triangle::new(0, 1, 2, RED),
				Triangle::new(0, 2, 3, RED),
				Triangle::new(4, 0, 3, GREEN),
				Triangle::new(4, 3, 7, GREEN),
				Triangle::new(5, 4, 7, BLUE),
				Triangle::new(5, 7, 6, BLUE),
				Triangle::new(1, 5, 6, YELLOW),
				Triangle::new(1, 6, 2, YELLOW),
				Triangle::new(4, 5, 1, PURPLE),
				Triangle::new(4, 1, 0, PURPLE),
				Triangle::new(2, 6, 7, CYAN),
				Triangle::new(2, 7, 3, CYAN),
			],
			verticies: [
				Vertex3::new( 1.,  1.,  1.),
				Vertex3::new(-1.,  1.,  1.),
				Vertex3::new(-1., -1.,  1.),
				Vertex3::new( 1., -1.,  1.),
				Vertex3::new( 1.,  1., -1.),
				Vertex3::new(-1.,  1., -1.),
				Vertex3::new(-1., -1., -1.),
				Vertex3::new( 1., -1., -1.),
			] 
		}
	}
}

#[derive(Debug)]
pub struct Model {
	pub triangles: Vec<Triangle>,
	pub verticies: Vec<Vertex3>
}

impl From<&Cube> for Model {
	fn from(cube: &Cube) -> Self {
		Model { triangles: Vec::from(cube.triangles), verticies: Vec::from(cube.verticies) }
	}
}

#[derive(Debug)]
pub struct Instance<'a> {
	pub model: &'a Model,
	pub position: Vertex3,
	pub orientation: Mat4x4,
	pub scale: f32,
	pub transform: Mat4x4
}

impl<'a> Instance<'a> {
	pub fn new(model: &'a Model, position: Vertex3, orientation: Option<Mat4x4>, scale: Option<f32>) -> Self {
		let mut s = 1.;
		let o;
		if let Some(scale) = scale {
			s = scale;
		}

		if let Some(orientation) = orientation {
			o = orientation;
		} else {
			o = IDEN_4X4.clone();
		}
		let t_mat = utils::make_translation_mat(position);
		let s_mat = utils::mul_mm(o, utils::make_scaling_mat(s));
		let transform = utils::mul_mm(t_mat, s_mat);

		Self {
			model,
			position,
			transform,
			orientation: o,
			scale: s,
		}
	}
}


