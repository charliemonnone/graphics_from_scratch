use std::{ops::{Add, Mul}, process::Output};
use glam::{mat4, vec4};
use macroquad::prelude::Color;

use super::{vec3::{Position, Vec3}, constants::*, utils};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Vertex {
	pub const fn new(x: f32, y: f32, z: f32) -> Self {
		Self {x, y, z}
	}
	
	pub fn translate(self, other: Self) -> Self {
		self + other
	}
}

impl From<&Vec3> for Vertex {

	fn from(vec3: &Vec3) -> Self {
		Vertex::new(vec3.v0, vec3.v1, vec3.v2)
	}
}

impl Add for Vertex {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Vertex::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl Add<&Vec3> for Vertex {
	type Output = Self;
	fn add(self, other: &Vec3) -> Self::Output {
		Vertex::new(self.x + other.v0, self.y + other.v1, self.z + other.v2)
	}
}

impl Add for &Vertex {
	type Output = Vertex;
	fn add(self, other: Self) -> Self::Output {
		Vertex::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl Mul<f32> for Vertex {
	type Output = Self;
	fn mul(self, k: f32) -> Self::Output {
		Vertex::new(self.x * k, self.y * k, self.z * k)
	}
}

impl Mul<f32> for &Vertex {
	type Output = Vertex;
	fn mul(self, k: f32) -> Self::Output {
		Vertex::new(self.x * k, self.y * k, self.z * k)
	}
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Point2 {
	pub x: i32,
	pub y: i32,
	pub h: f32
}

impl Point2 {

	pub fn add(self, x: i32, y: i32) -> Self {
		Self::new(self.x + x, self.y + y, self.h)
	}
	
	pub const fn new(x: i32, y: i32, h: f32) -> Self {
		Self { x, y, h }
	}

}

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
	pub verticies: [Vertex; 8]
}

impl Cube {
	pub const fn debug_cube() -> Self {
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
				Vertex::new( 1.,  1.,  1.),
				Vertex::new(-1.,  1.,  1.),
				Vertex::new(-1., -1.,  1.),
				Vertex::new( 1., -1.,  1.),
				Vertex::new( 1.,  1., -1.),
				Vertex::new(-1.,  1., -1.),
				Vertex::new(-1., -1., -1.),
				Vertex::new( 1., -1., -1.),
			] 
		}
	}
}

#[derive(Debug)]
pub struct Model {
	pub triangles: Vec<Triangle>,
	pub verticies: Vec<Vertex>
}

impl From<&Cube> for Model {
	fn from(cube: &Cube) -> Self {
		Model { triangles: Vec::from(cube.triangles), verticies: Vec::from(cube.verticies) }
	}
}

#[derive(Debug)]
pub struct Instance<'a> {
	pub model: &'a Model,
	pub position: Position,
	pub orientation: Mat4x4,
	pub scale: f32,
	pub transform: Mat4x4
}

impl<'a> Instance<'a> {
	pub fn new(model: &'a Model, position: Position, orientation: Option<Mat4x4>, scale: Option<f32>) -> Self {
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
		let t_mat = utils::make_translation_mat(position);//.transpose();
		let s_mat = utils::mul_mm(o, utils::make_scaling_mat(s));
		// println!("t row1: {}", t_mat.row(0));
		// println!("t row2: {}", t_mat.row(1));
		// println!("t row3: {}", t_mat.row(2));
		// println!("t row4: {}", t_mat.row(3));
		// println!("");
		// println!("s row1: {}", s_mat.row(0));
		// println!("s row2: {}", s_mat.row(1));
		// println!("s row3: {}", s_mat.row(2));
		// println!("s row4: {}", s_mat.row(3));
		// println!("");
		let transform = utils::mul_mm(t_mat, s_mat);//.transpose();

		Self {
			model,
			position,
			transform,
			orientation: o,
			scale: s,
		}
	}
}


pub type Point3 = glam::Vec3;
pub type Vertex4 = glam::Vec4;
pub type Vec4 = glam::Vec4;
pub type Mat4x4 = glam::Mat4;
pub const IDEN_4X4: Mat4x4 = glam::const_mat4!(
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0]
);