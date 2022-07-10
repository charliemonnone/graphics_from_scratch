use macroquad::{prelude::*, window::Conf};

mod raytracer;
mod rasterizer;

fn window_conf() -> Conf {
    Conf {
        window_title: "Graphics From Scratch".to_owned(),
        window_width: 800,
        window_height: 640,
        ..Default::default()
    }
}

enum Program {
	RayTracer,
	Rasterizer
}

async fn rt(init_width: f32, init_height: f32) {
    let mut width = init_width;
    let mut height = init_height;
	let mut resize_texture = false;
    let mut texture = render_raytracer_scene(width, height);

    loop {
        if width != screen_width() || height != screen_height() {
            resize_texture = !resize_texture;
        }
        if resize_texture {
            resize_texture = !resize_texture;
            width = screen_width();
            height = screen_height();
            texture.delete();
            texture = render_raytracer_scene(width, height);
        }
        draw_screen(texture);
        draw_stats(WHITE);
        next_frame().await
    }
}

async fn rast(init_width: f32, init_height: f32) {
    let mut width = init_width;
    let mut height = init_height;
	let mut resize_texture = false;

    let mut texture = render_rasterizer_scene(width, height);
    loop {
        if width != screen_width() || height != screen_height() {
            resize_texture = !resize_texture;
        }
        if resize_texture {
            resize_texture = !resize_texture;
            width = screen_width();
            height = screen_height();
            texture.delete();
            texture = render_rasterizer_scene(width, height);
        }
        draw_screen(texture);
        draw_stats(BLACK);

        next_frame().await
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = screen_width();
    let height = screen_height();
	let program = Program::Rasterizer;
    
    
    match program {
		Program::RayTracer => rt(width, height).await,
		Program::Rasterizer => rast(width, height).await,

	}

}

fn draw_stats(color: Color) {
    let fps = format!("fps: {}", get_fps());
    let frametime = format!("frame time: {}", get_frame_time());
    draw_text(fps.as_str(), screen_width() * 0.01, screen_height() * 0.03, 24.0, color);
    draw_text(frametime.as_str(), screen_width() * 0.01, (screen_height() * 0.03)+20.0, 24.0, color);
}

fn draw_screen(buffer: Texture2D) {
    clear_background(WHITE);
    draw_texture(
        buffer,
        screen_width() / 2. - buffer.width() / 2.,
        screen_height() / 2. - buffer.height() / 2.,
        WHITE,
    );
}

fn render_raytracer_scene(width: f32, height: f32) -> Texture2D {
    let mut image = Image::gen_image_color(width as u16, height as u16, WHITE);
    raytracer::render::run(&mut image, width, height);
    Texture2D::from_image(&image)
}

fn render_rasterizer_scene(width: f32, height: f32) -> Texture2D {
    let mut image = Image::gen_image_color(width as u16, height as u16, WHITE);
    rasterizer::render::run(&mut image, width, height);
    Texture2D::from_image(&image)
}