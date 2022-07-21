use macroquad::prelude::{Color, RED, GREEN, BLUE, YELLOW, PURPLE};

use super::{vertex::Vertex, vec3::Position};

const CYAN: Color = Color::new(0., 1., 1., 1.);

#[derive(Debug, Default, Copy, Clone)]
pub struct Triangle {
	pub v0: u32,
	pub v1: u32,
	pub v2: u32,
	pub color: Color
}

impl Triangle {
	const fn new(v0: u32, v1: u32, v2: u32, color: Color) -> Self {
		Self { v0, v1, v2, color }
	}
}

pub enum ModelType {
	Cube
}

pub struct ModelInstance {
	model: ModelType,
	position: Position
}

impl ModelInstance {

}

#[derive(Debug, Default, Clone)]
pub struct Cube {
	pub triangles: [Triangle; 12],
	pub verticies: [Vertex; 8]
}
impl Cube {
	const fn debug_cube() -> Self {
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

pub static CUBE: Cube = Cube::debug_cube();
