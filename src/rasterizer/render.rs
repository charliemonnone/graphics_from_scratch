use super::{
    camera::Camera,
    data_types::Instance,
    data_types::{Point, Vertex3},
    data_types::{Model, Triangle, Vertex4},
    clipping::transform_and_clip,
    main::{get_canvas_dimensions, get_draw_outline},
    utils::{self, interpolate, math::INFINITY_F32, mul_color},
};
use macroquad::{color::Color, texture::Image};
use std::mem::swap;

pub fn draw_line(image: &mut Image, p0: Point, p1: Point, color: Color) {
    let mut p0 = p0;
    let mut p1 = p1;

    let mut x0 = p0.x;
    let mut y0 = p0.y;
    let mut x1 = p1.x;
    let mut y1 = p1.y;

    let dx = x1 - x0;
    let dy = y1 - y0;

    if utils::math::abs(dx) > utils::math::abs(dy) {
        // horizontalish
        if x0 > x1 {
            swap(&mut p0, &mut p1);
            x0 = p0.x;
            y0 = p0.y;
            x1 = p1.x;
            y1 = p1.y;
        }
        let ys = utils::interpolate(x0, y0 as f32, x1, y1 as f32);
        let start = x0 as i32;
        let end = x1 as i32;
        for x in start..=end {
            let index: usize = (x as f32 - x0) as usize;
            let y = ys[index];
            put_pixel(image, x, y as i32, color);
        }
    } else {
        // verticalish
        if y0 > y1 {
            swap(&mut p0, &mut p1);
            x0 = p0.x;
            y0 = p0.y;
            x1 = p1.x;
            y1 = p1.y;
        }
        let xs = utils::interpolate(y0, x0 as f32, y1, x1 as f32);
        let start = y0 as i32;
        let end = y1 as i32;
        for y in start..=end {
            let index: usize = (y as f32 - y0) as usize;
            let x = xs[index];
            put_pixel(image, x as i32, y, color);
        }
    }
}

fn sorted_vertex_indicies(triangle: &Triangle, projected: &Vec<Point>) -> [usize; 3] {
    let mut ind = [0, 1, 2];
    let verticies = [triangle.ind[0], triangle.ind[1], triangle.ind[2]];
    if projected[verticies[ind[1]]].y < projected[verticies[ind[0]]].y {
        let swap = ind[0];
        ind[0] = ind[1];
        ind[1] = swap;
    }

    if projected[verticies[ind[2]]].y < projected[verticies[ind[0]]].y {
        let swap = ind[0];
        ind[0] = ind[2];
        ind[2] = swap;
    }

    if projected[verticies[ind[2]]].y < projected[verticies[ind[1]]].y {
        let swap = ind[1];
        ind[1] = ind[2];
        ind[2] = swap;
    }


    ind
}

fn compute_triangle_normal(v0: Vertex3, v1: Vertex3, v2: Vertex3) -> Vertex3 {
    let v0_v1 = v1 + (v0 * -1.);
    let v0_v2 = v2 + (v0 * -1.);
    v0_v1.cross(v0_v2)

}

fn edge_interpolate(y0: f32, v0: f32, y1: f32, v1: f32, y2: f32, v2: f32) -> (Vec<f32>, Vec<f32>) {
    let mut v01 = interpolate(y0, v0, y1, v1);
    let mut v12 = interpolate(y1, v1, y2, v2);
    let v02 = interpolate(y0, v0, y2, v2);
    v01.pop();
    v01.append(&mut v12);

    (v02, v01)
}

fn update_depth_buffer_is_closer(depth_buf: &mut Vec<f32>, x: isize, y: isize, inv_z: f32) -> bool {
    let (width, height) = get_canvas_dimensions();
    let width = width as isize;
    let height = height as isize;
    let canv_x = (width / 2) + x;
    let canv_y = (height / 2) + y  - 1;

    if canv_x < 0 || canv_x >= width || canv_y < 0 || canv_y >= height {
        return false;
    }

    let offset = (canv_x + (width * canv_y)) as usize;
    if depth_buf[offset] == INFINITY_F32 || depth_buf[offset] < inv_z {
        depth_buf[offset] = inv_z;
        return true; 
    }

    false
}

fn render_triangle(image: &mut Image, triangle: &Triangle, verticies: &Vec<Vertex3>, projected: &Vec<Point>, depth_buf: &mut Vec<f32>) {
    let indicies = sorted_vertex_indicies(triangle, projected);
    let unsorted = triangle.ind;
    let i0 = indicies[0];
    let i1 = indicies[1];
    let i2 = indicies[2];

    let v0 = verticies[triangle.ind[i0]];
    let v1 = verticies[triangle.ind[i1]];
    let v2 = verticies[triangle.ind[i2]];
    
    let normal = compute_triangle_normal(verticies[unsorted[0]], verticies[unsorted[1]], verticies[unsorted[2]]);
    let center = (-1. / 3.) * ((verticies[unsorted[0]] + verticies[unsorted[1]]) + verticies[unsorted[2]]);
    if center.dot(normal) < 0. {
        return;
    }
    
    // Get attribute values (X, 1/Z) at the vertices.
    let p0 = projected[unsorted[i0]];
    let p1 = projected[unsorted[i1]];
    let p2 = projected[unsorted[i2]];

    // Compute attribute values at the edges.
    let (x02, x012) = edge_interpolate(p0.y, p0.x, p1.y, p1.x, p2.y, p2.x);
    let (iz02, iz012) = edge_interpolate(p0.y, 1./v0.z, p1.y, 1./v1.z, p2.y, 1./v2.z);

    // Determine which is left and which is right.
    let m = x02.len() / 2;
    let x_left: Vec<f32>;
    let x_right: Vec<f32>;
    let iz_left: Vec<f32>;
    let iz_right: Vec<f32>;

    if &x02[m] < &x012[m] {
        x_left = x02;
        x_right = x012;
        iz_left = iz02;
        iz_right = iz012;
    } else {
        x_left = x012;
        x_right = x02;
        iz_left = iz012;
        iz_right = iz02;
    }

    let start = p0.y as isize;
    let end = p2.y as isize;

    // Draw horizontal segments.
    for y in start..=end {
        let y_ind = (y - start) as usize;
        let (xl, xr) = (x_left[y_ind], x_right[y_ind]);
        let (zl, zr) = (iz_left[y_ind], iz_right[y_ind]);
        let z_scan = interpolate(xl, zl, xr, zr);
        let x_start = xl as isize;
        let x_end = xr as isize;

        for x in x_start..=x_end {
            let x_ind = (x - x_start) as usize;
            if update_depth_buffer_is_closer(depth_buf, x, y, z_scan[x_ind]) {
                put_pixel(image, x as i32, y as i32, triangle.color)
            }
        }
    }

    let draw_outline = get_draw_outline();
    if draw_outline {
        let outline_color = mul_color(&triangle.color, 0.75);
        draw_line(image, p0, p1, outline_color);
        draw_line(image, p0, p2, outline_color);
        draw_line(image, p2, p1, outline_color);
    }
}

pub fn render_scene(image: &mut Image, cam: &Camera, instances: &Vec<Instance>, depth_buf: &mut Vec<f32>) {
    let camera_mat = utils::mul_mm(
        cam.orientation.transpose(),
        utils::make_translation_mat(cam.pos * -1.),
    );

    for i in instances{
        let transform = utils::mul_mm(camera_mat, i.transform);
        if let Some(clipped) = transform_and_clip(&cam.clipping_planes, i.model, transform) {
            render_model(image, clipped, depth_buf);
        } 
    }
}

pub fn render_model(image: &mut Image, model: Model, depth_buf: &mut Vec<f32>) {
    let mut projected: Vec<Point> = vec![];
    for v in &model.verticies {
        let v = Vertex4::new(v.x, v.y, v.z, 1.);

        let mut p_v = utils::project_vertex(v);
        utils::truncate_parts(&mut p_v);
        projected.push(p_v);
    }
    for t in &model.triangles {
        render_triangle(image, t, &model.verticies, &projected, depth_buf);
    }
}

fn put_pixel(image: &mut Image, x: i32, y: i32, color: Color) {
    let width = image.width();
    let height = image.height();
    let (x_mapped, y_mapped) = utils::map_to_pixels(x, y, width, height);

    if x_mapped >= width || y_mapped >= height {
        return;
    }

    image.set_pixel(x_mapped as u32, y_mapped as u32, color);
}
