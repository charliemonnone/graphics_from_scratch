mod ray_tracer;
mod color;
mod canvas;
mod math;
mod scene;
mod point;
mod sphere;
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

	let width = screen_width();
	let height = screen_height();
	let mut image = Image::gen_image_color(width as u16, height as u16, WHITE);

	ray_tracer::run(&mut image, width, height);
	let texture = Texture2D::from_image(&image);

	loop {
        clear_background(BLACK);
		
		draw_texture(
            texture,
            screen_width() / 2. - texture.width() / 2.,
            screen_height() / 2. - texture.height() / 2.,
            WHITE,
        );

		let fps = format!("{}", get_fps());
		draw_text(fps.as_str(), 30.0, 30.0, 30.0, BLACK);
        next_frame().await
    }
}