use std::{ops::{Add, Mul}, process::Output};

use macroquad::prelude::Color;

use super::{vec3::{Position, Vec3}, constants::*};

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

pub struct Model {
	pub triangles: Vec<Triangle>,
	pub verticies: Vec<Vertex>
}

impl From<&Cube> for Model {
	fn from(cube: &Cube) -> Self {
		Model { triangles: Vec::from(cube.triangles), verticies: Vec::from(cube.verticies) }
	}
}

pub struct Instance<'a> {
	pub model: &'a Model,
	pub position: Position
}

impl<'a> Instance<'a> {
	pub fn new(model: &'a Model, position: Position) -> Self {
		Self {
			model,
			position
		}
	}
}