#![allow(dead_code, unused_variables, non_snake_case)]

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod point;
use point::*;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 48;
pub const SCALE: u32 = 10;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
type Display =  [(u8, u8, u8); (WIDTH * HEIGHT) as usize];

fn draw_loop(d: &mut Display) {
	static mut t: u32 = 0;
	unsafe { t += 1; }

	for i in 0..(WIDTH*HEIGHT) as usize {
		let perc = i as f32 / (WIDTH*HEIGHT) as f32;
		d[i] = ((i as u32 % WIDTH) as u8, 0, 0);
	}

	unsafe {
		let v0 = Vertex::vertex(0.0, 0.7, 0.0);
		let v1 = Vertex::vertex(-0.7, -0.0 + 0.5 * (t as f32 / 10.0).sin(), 0.0);
		let v2 = Vertex::vertex(0.7, -0.7, 0.0);
		let v3 = Vertex::vertex(-1.0, -0.7, 0.0);
		let v4 = Vertex::vertex(0.0, -1.0, 0.0);
		draw_tri(d, v0, v1, v2);

		draw_tri(d, v3, v1, v2);

		draw_tri(d, v3, v4, v2);
	}
}

fn draw_scanline(d: &mut Display, a: f32, b: f32, y: u32, c1: Point3D, c2: Point3D) {
	let (min, max): (u32, u32);
	if a > b { min = b.round() as u32; max = a.round() as u32; }
	else { min = a.round() as u32; max = b.round() as u32; }
	for x in min..max {
		let inter = (x - min) as f32 / (max - min) as f32;
		let c = lerp3(c1, c2, inter);
		d[(y * WIDTH + x) as usize].0 = (c.x*255.0) as u8;
		d[(y * WIDTH + x) as usize].1 = (c.y*255.0) as u8;
		d[(y * WIDTH + x) as usize].2 = (c.z*255.0) as u8;
	}
}

fn draw_tri(d: &mut Display, a: Vertex, b: Vertex, c: Vertex) {
	let mut list = [toDCoords(a.point2D()), toDCoords(b.point2D()),
	    toDCoords(c.point2D())];
	if list[0].y > list[1].y { list.swap(0, 1); } // Bubblesort lol
	if list[0].y > list[2].y { list.swap(0, 2); }
	if list[1].y > list[2].y { list.swap(1, 2); }

	let midV = (list[1].y - list[0].y) / (list[2].y - list[0].y);
	let mid = lerp(list[0].x as f32, list[2].x as f32, midV);

	for y in list[0].y.round() as u32..list[1].y.round() as u32 {
		let v_a = (y as f32 - list[0].y) / (list[2].y - list[0].y);
		let v_b = (y as f32 - list[0].y) / (list[1].y - list[0].y);
		let a = lerp(list[0].x as f32, list[2].x as f32, v_a);
		let b = lerp(list[0].x as f32, list[1].x, v_b);
		draw_scanline(d, a, b, y, point3D(0.0,0.0,0.0), point3D(1.0,1.0,1.0));
	}
	for y in list[1].y.round() as u32..list[2].y.round() as u32 {
		let v_a = (y as f32 - list[1].y) / (list[2].y - list[1].y);
		let v_b = (y as f32 - list[1].y) / (list[2].y - list[1].y);
		let a = lerp(list[1].x as f32, list[2].x as f32, v_a);
		let b = lerp(mid, list[2].x as f32, v_b);
		draw_scanline(d, a, b, y, point3D(0.0,0.0,0.0), point3D(1.0,1.0,1.0));
	}

	d[(list[0].y as u32 * WIDTH + list[0].x as u32) as usize].2 = 255;
	d[(list[1].y as u32 * WIDTH + list[1].x as u32) as usize].2 = 255;
	d[(list[2].y as u32 * WIDTH + list[2].x as u32) as usize].2 = 255;
}

fn draw_pixel(canvas: &mut Canvas, x: u32, y: u32, rgb: Color) {
	canvas.set_draw_color(rgb);
	canvas.fill_rect(sdl2::rect::Rect::new((x*SCALE) as i32, (y*SCALE) as i32,
		SCALE as u32, SCALE as u32)).expect("Couldn't draw pixel");
}

fn create_window() -> (sdl2::Sdl, sdl2::VideoSubsystem, sdl2::render::Canvas<sdl2::video::Window>) {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("Rust 3D Test",
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
		
		draw_loop(&mut display);

		for y in 0..HEIGHT {
			for x in 0..WIDTH {
				let c = display[((HEIGHT - y - 1) * WIDTH + x) as usize];
				draw_pixel(&mut canvas, x, y, Color::RGB(c.0, c.1, c.2));
			}
		}

		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}

