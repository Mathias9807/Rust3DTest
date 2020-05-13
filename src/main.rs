#![allow(dead_code, unused_variables, non_snake_case)]

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;
use std::io::Write;

mod point;
mod model;
mod graphics;
use point::*;
use model::*;
use graphics::*;

pub const WINDOW_W: u32 = 640;
pub const WINDOW_H: u32 = 480;
pub const SCALE: u32 = 1;
pub const WIDTH: u32 = WINDOW_W / SCALE;
pub const HEIGHT: u32 = WINDOW_H / SCALE;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

fn draw_loop(state: &mut GraphicsState, teapot: &Model) {
	static mut T: u32 = 0;
	unsafe { T += 1; }

	for i in 0..(WIDTH*HEIGHT) as usize {
		let perc = i as f32 / (WIDTH*HEIGHT) as f32;
		state.d[i] = ((i as u32 * 256 / (WIDTH*HEIGHT)) as u8, 0, 0);
	}

    unsafe {
        // let mat = Mat4::identity().translate3f((T as f32).sin(), (T as f32).cos(), 0.);
        let mat = Mat4::identity().rotate(Vec3f([1.,1.,1.]), (T as f32)/5.);

        state.draw_model(teapot, mat);
    }

	// unsafe {
	// 	let v0 = Vertex::new(0.0, 0.7, 0.0);
	// 	let v1 = Vertex::new(-0.7, -0.0 + 0.5 * (T as f32 / 100.0).sin(), 0.0);
	// 	let v2 = Vertex::new(0.7, -0.7, 0.0);
	// 	let v3 = Vertex::new(-1.0, -0.7, 0.0);
	// 	let v4 = Vertex::new(0.0, -1.0, 0.0);
	// 	draw_tri(d, v0, v1, v2);

	// 	draw_tri(d, v3, v1, v2);

	// 	draw_tri(d, v3, v4, v2);
	// }
}


fn draw_pixel(canvas: &mut Canvas, x: u32, y: u32, rgb: Color) {
	canvas.set_draw_color(rgb);
	canvas.fill_rect(sdl2::rect::Rect::new((x*SCALE) as i32, (y*SCALE) as i32,
		SCALE as u32, SCALE as u32)).expect("Couldn't draw pixel");
}

fn create_window() -> (sdl2::Sdl, sdl2::VideoSubsystem, sdl2::render::Canvas<sdl2::video::Window>) {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("Rust 3D Test", WINDOW_W, WINDOW_H)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();

	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.present();

	return (sdl_context, video_subsystem, canvas);
}

fn main() {
	let (sdl_context, _video_subsystem, mut canvas) = create_window();

	let mut display = [(0u8, 0u8, 0u8); (WIDTH * HEIGHT) as usize];

    let mut state = GraphicsState{d: &mut display};

	let teapot: Model = Model::load_obj("res/teapot.obj");

    VecTest();

	let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frames: u32 = 0;
    let mut now = Instant::now();
	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				},
				_ => {}
			}
		}
		
		draw_loop(&mut state, &teapot);

		for y in 0..HEIGHT {
			for x in 0..WIDTH {
				let c = state.d[((HEIGHT - y - 1) * WIDTH + x) as usize];
				draw_pixel(&mut canvas, x, y, Color::RGB(c.0, c.1, c.2));
			}
		}

		canvas.present();
        frames += 1;

        // Check how long it's been since last FPS count
        let elapsed = now.elapsed().as_millis() as f32 / 1000.;
        if elapsed > 1. {
            print!("\r\u{001b}[2KFPS: {},\tSPF: {}", frames as f32 / elapsed,
                   elapsed / frames as f32);
            std::io::stdout().flush().unwrap();
            now = Instant::now();
            frames = 0;
        }
	}
    println!("");
}

