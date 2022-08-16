use super::{data_types::{Model, Mat4x4, Plane, Vertex4, Triangle, Vertex3}, utils::mul_mv};

pub fn transform_and_clip(planes: &[Plane], model: &Model, transform: Mat4x4) -> Option<Model> {
	let center = mul_mv(transform, model.bounds_center);
	let radius_sq = model.bounds_radius * model.bounds_radius;
	for p in planes {
		let dist = p.normal.dot(center.truncate()) + p.dist;
		if dist < -radius_sq {
			return None;
		}
	}
	
	let verticies: Vec<Vertex3> = model.verticies
		.iter()
		.map(|v| mul_mv(transform, Vertex4::new(v.x, v.y, v.z, 1.)).truncate() )
		.collect();
	let mut triangles: Vec<Triangle> = model.triangles.clone();

	for p in planes {
		let mut new_triangles: Vec<Triangle> = vec![];
		for t in &triangles {
			clip_triangle(t, p, &mut new_triangles, &verticies);
		}
		triangles = new_triangles.clone()
	}
	Some(Model::new(triangles, verticies, center, model.bounds_radius))

}

pub fn clip_triangle(triangle: &Triangle, plane: &Plane, tris: &mut Vec<Triangle>, verts: &Vec<Vertex3>) {
	let v0 = verts[triangle.ind[0]];
	let v1 = verts[triangle.ind[0]];
	let v2 = verts[triangle.ind[0]];
	
	let in0 = plane.normal.dot(v0) + plane.dist > 0.;
	let in1 = plane.normal.dot(v1) + plane.dist > 0.;
	let in2 = plane.normal.dot(v2) + plane.dist > 0.;

	let in_count = i32::from(in0) + i32::from(in1) + i32::from(in2);
	match in_count {
		3 => tris.push(triangle.clone()),
		_ => {}
	}
}