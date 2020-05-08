#![allow(dead_code, unused_variables, non_snake_case)]

use std::ops;

use crate::{WIDTH, HEIGHT};

#[derive(Copy, Clone, Debug)]
pub struct Point2D {
	pub x: f32,
	pub y: f32
}
impl ops::Add<Point2D> for Point2D {
	type Output = Point2D;
	fn add(self, rhs: Point2D) -> Point2D {
		point2D(self.x+rhs.x, self.y+rhs.y) }
}
impl ops::Sub<Point2D> for Point2D {
	type Output = Point2D;
	fn sub(self, rhs: Point2D) -> Point2D {
		point2D(self.x-rhs.x, self.y-rhs.y) }
}
impl ops::Mul<Point2D> for Point2D {
	type Output = Point2D;
	fn mul(self, rhs: Point2D) -> Point2D {
		point2D(self.x*rhs.x, self.y*rhs.y) }
}
impl ops::Mul<f32> for Point2D {
	type Output = Point2D;
	fn mul(self, rhs: f32) -> Point2D {
		point2D(self.x*rhs, self.y*rhs) }
}
impl ops::Div<Point2D> for Point2D {
	type Output = Point2D;
	fn div(self, rhs: Point2D) -> Point2D {
		point2D(self.x/rhs.x, self.y/rhs.y) }
}
impl ops::Div<f32> for Point2D {
	type Output = Point2D;
	fn div(self, rhs: f32) -> Point2D {
		point2D(self.x/rhs, self.y/rhs) }
}
pub const fn point2D(x: f32, y: f32) -> Point2D { Point2D{x: x, y: y} }

pub const MIN_SCREEN: Point2D = point2D(-5.0, -5.0);
pub const MAX_SCREEN: Point2D = point2D( 5.0,  5.0);

pub fn toDCoords(p: Point2D) -> Point2D {
	(p - MIN_SCREEN) / (MAX_SCREEN - MIN_SCREEN)
	* point2D(WIDTH as f32, HEIGHT as f32)
}

#[derive(Copy, Clone, Debug)]
pub struct Point3D {
	pub x: f32,
	pub y: f32,
	pub z: f32
}
impl ops::Add<Point3D> for Point3D {
	type Output = Point3D;
	fn add(self, rhs: Point3D) -> Point3D {
		point3D(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z) }
}
impl ops::Sub<Point3D> for Point3D {
	type Output = Point3D;
	fn sub(self, rhs: Point3D) -> Point3D {
		point3D(self.x-rhs.x, self.y-rhs.y, self.z-rhs.z) }
}
impl ops::Mul<Point3D> for Point3D {
	type Output = Point3D;
	fn mul(self, rhs: Point3D) -> Point3D {
		point3D(self.x*rhs.x, self.y*rhs.y, self.z*rhs.z) }
}
impl ops::Mul<f32> for Point3D {
	type Output = Point3D;
	fn mul(self, rhs: f32) -> Point3D {
		point3D(self.x*rhs, self.y*rhs, self.z*rhs) }
}
impl ops::Div<Point3D> for Point3D {
	type Output = Point3D;
	fn div(self, rhs: Point3D) -> Point3D {
		point3D(self.x/rhs.x, self.y/rhs.y, self.z/rhs.z) }
}
impl ops::Div<f32> for Point3D {
	type Output = Point3D;
	fn div(self, rhs: f32) -> Point3D {
		point3D(self.x/rhs, self.y/rhs, self.z/rhs) }
}
pub const fn point3D(x: f32, y: f32, z: f32) -> Point3D { Point3D{x: x, y: y, z: z} }
impl Point3D {
	pub fn point2D(&self) -> Point2D { point2D(self.x, self.y) }
}

pub fn lerp3(a: Point3D, b: Point3D, v: f32) -> Point3D {
	point3D(lerp(a.x, b.x, v), lerp(a.y, b.y, v), lerp(a.z, b.z, v))
}
pub fn lerp(a: f32, b: f32, v: f32) -> f32 {
	if v < 0.0 { return a; }
	if v > 1.0 { return b; }
	return a * (1.0-v) + b * v;
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
	pub p: Point3D,
	pub c: Point3D
}
impl Vertex {
	pub fn vertex(x: f32, y: f32, z: f32) -> Vertex {
		Vertex{p: point3D(x, y, z), c: point3D(1.0, 1.0, 1.0)}
	}
	pub fn color(&mut self, r: f32, g: f32, b: f32) -> &Vertex {
		self.c = point3D(r, g, b); self
	}
	pub fn point2D(&self) -> Point2D { point2D(self.p.x, self.p.y) }
}

