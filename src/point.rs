use std::ops::{Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Default for Point3 {
	fn default() -> Self { Self { x: 0.0, y: 0.0, z: 0.0 } }
}

impl Sub for Point3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
			z: self.z - other.z
        }
    }
}

impl Sub for &Point3 {
	type Output = Point3;

    fn sub(self, other: Self) -> Self::Output {
        Point3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul for &Point3 {
	type Output = Point3;

    fn mul(self, other: Self) -> Self::Output {
        Point3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Mul for Point3 {
	type Output = Point3;

    fn mul(self, other: Self) -> Self::Output {
        Point3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}


impl Point3 {
	pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
}



// impl Is3D for Point3 {
//     fn x(&self) -> f32 { self.x }
//     fn y(&self) -> f32 { self.y }
//     fn z(&self) -> f32 { self.z }

//     fn sub(&self, other: &impl Is3D) -> Self {
//         Point3 { x: self.x() - other.x(), y: self.y() - other.y(), z: self.z() - other.z() }

//     }

//     fn mul(&self, other: &impl Is3D) -> Self {
//         Point3 { x: self.x() * other.x(), y: self.y() * other.y(), z: self.z() * other.z() }
//     }

// } 

// pub trait Is3D {
//     fn x(&self) -> f32;
//     fn y(&self) -> f32;
//     fn z(&self) -> f32;
//     fn sub(&self, other: &impl Is3D) -> Self;
//     fn mul(&self, other: &impl Is3D) -> Self;
// }