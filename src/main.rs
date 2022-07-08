mod canvas;
mod color;
mod light;
mod math;
mod ray_tracer;
mod scene;
mod sphere;
mod vec3;
mod camera;
mod mat3;

use macroquad::{prelude::*, window::Conf};

fn window_conf() -> Conf {
    Conf {
        window_title: "Graphics From Scratch".to_owned(),
        window_width: 800,
        window_height: 640,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut width = screen_width();
    let mut height = screen_height();
    let mut resize_texture = false;
    let mut texture = render_scene(width, height);

    loop {
        if width != screen_width() || height != screen_height() {
            resize_texture = !resize_texture;
        }
        if resize_texture {
            resize_texture = !resize_texture;
            width = screen_width();
            height = screen_height();
            texture.delete();
            texture = render_scene(width, height);
        }
        draw_screen(texture);
        draw_stats();
        next_frame().await
    }
}

fn draw_stats() {
    let fps = format!("fps: {}", get_fps());
    let frametime = format!("frame time: {}", get_frame_time());
    draw_text(fps.as_str(), screen_width() * 0.01, screen_height() * 0.03, 24.0, BLACK);
    draw_text(frametime.as_str(), screen_width() * 0.01, (screen_height() * 0.03)+20.0, 24.0, BLACK);
}

fn draw_screen(buffer: Texture2D) {
    // clear_background(BLACK);
    draw_texture(
        buffer,
        screen_width() / 2. - buffer.width() / 2.,
        screen_height() / 2. - buffer.height() / 2.,
        WHITE,
    );
}

fn render_scene(width: f32, height: f32) -> Texture2D {
    let mut image = Image::gen_image_color(width as u16, height as u16, WHITE);
    ray_tracer::run(&mut image, width, height);
    Texture2D::from_image(&image)
}