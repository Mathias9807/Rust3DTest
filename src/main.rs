#![allow(dead_code, unused_variables)]

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 48;
const SCALE: u32 = 10;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
type Display =  [(u8, u8, u8); (WIDTH * HEIGHT) as usize];

fn draw_loop(d: &mut Display) {
	d[0] = (255, 0, 0);
}

fn draw_pixel(canvas: &mut Canvas, x: u32, y: u32, rgb: Color) {
	canvas.set_draw_color(rgb);
	canvas.fill_rect(sdl2::rect::Rect::new((x*SCALE) as i32, (y*SCALE) as i32,
		SCALE as u32, SCALE as u32)).expect("Couldn't draw pixel");
}

fn create_window() -> (sdl2::Sdl, sdl2::VideoSubsystem, sdl2::render::Canvas<sdl2::video::Window>) {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("RustRaytracer",
		WIDTH*SCALE, HEIGHT*SCALE)
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

	draw_loop(&mut display);

	let mut event_pump = sdl_context.event_pump().unwrap();
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
		
		for y in 0..HEIGHT {
			for x in 0..WIDTH {
				let c = display[(y * HEIGHT + x) as usize];
				draw_pixel(&mut canvas, x, y, Color::RGB(c.0, c.1, c.2));
			}
		}
		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}

