#![allow(dead_code, unused_variables, non_snake_case)]

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod point;
use point::*;
mod model;
use model::*;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;
pub const SCALE: u32 = 1;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
type Display =  [(u8, u8, u8); (WIDTH * HEIGHT) as usize];

fn draw_loop(d: &mut Display, teapot: &Model) {
	static mut T: u32 = 0;
	unsafe { T += 1; }

	for i in 0..(WIDTH*HEIGHT) as usize {
		let perc = i as f32 / (WIDTH*HEIGHT) as f32;
		d[i] = ((i as u32 * 256 / (WIDTH*HEIGHT)) as u8, 0, 0);
	}

	draw_model(d, teapot);

	unsafe {
		// let v0 = Vertex::vertex(0.0, 0.7, 0.0);
		// let v1 = Vertex::vertex(-0.7, -0.0 + 0.5 * (T as f32 / 100.0).sin(), 0.0);
		// let v2 = Vertex::vertex(0.7, -0.7, 0.0);
		// let v3 = Vertex::vertex(-1.0, -0.7, 0.0);
		// let v4 = Vertex::vertex(0.0, -1.0, 0.0);
		// draw_tri(d, v0, v1, v2);

		// draw_tri(d, v3, v1, v2);

		// draw_tri(d, v3, v4, v2);
	}
}

fn draw_model(d: &mut Display, model: &Model) {
	for f in &model.faces {
		draw_tri(d, model.verts[f.0], model.verts[f.1], model.verts[f.2]);
	}
}

fn draw_scanline(d: &mut Display, a: f32, b: f32, y: u32, c1: Point3D, c2: Point3D) {
	let (mut min, mut max): (i32, i32);
	if a > b { min = b.round() as i32; max = a.round() as i32; }
	else { min = a.round() as i32; max = b.round() as i32; }
	if y >= HEIGHT { return }
	min = min.max(0);
	max = max.min(WIDTH as i32 - 1);
	for x in min..max {
		let inter = (x - min) as f32 / (max - min) as f32;
		let c = lerp3(c1, c2, inter);
		d[(y * WIDTH + x as u32) as usize].0 = (c.x*255.0) as u8;
		d[(y * WIDTH + x as u32) as usize].1 = (c.y*255.0) as u8;
		d[(y * WIDTH + x as u32) as usize].2 = (c.z*255.0) as u8;
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

	for y in list[0].y.round().max(0.0) as u32..list[1].y.round() as u32 {
		let v_a = (y as f32 - list[0].y) / (list[2].y - list[0].y);
		let v_b = (y as f32 - list[0].y) / (list[1].y - list[0].y);
		let a = lerp(list[0].x as f32, list[2].x as f32, v_a);
		let b = lerp(list[0].x as f32, list[1].x, v_b);
		draw_scanline(d, a, b, y, point3D(0.0,0.0,0.0), point3D(1.0,1.0,1.0));
	}
	for y in list[1].y.round().max(0.0) as u32..list[2].y.round() as u32 {
		let v_a = (y as f32 - list[1].y) / (list[2].y - list[1].y);
		let v_b = (y as f32 - list[1].y) / (list[2].y - list[1].y);
		let a = lerp(list[1].x as f32, list[2].x as f32, v_a);
		let b = lerp(mid, list[2].x as f32, v_b);
		draw_scanline(d, a, b, y, point3D(0.0,0.0,0.0), point3D(1.0,1.0,1.0));
	}
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

	let teapot: Model = Model::load_obj("res/teapot.obj");

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
		
		draw_loop(&mut display, &teapot);

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

