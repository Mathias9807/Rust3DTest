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

pub const MIN_SCREEN: Point2D = point2D(-1.0, -1.0);
pub const MAX_SCREEN: Point2D = point2D( 1.0,  1.0);

pub fn toDCoords(p: Point2D) -> Point2D {
	(p - MIN_SCREEN) / (MAX_SCREEN - MIN_SCREEN)
	* point2D(WIDTH as f32, HEIGHT as f32)
}

pub fn lerp(a: f32, b: f32, v: f32) -> f32 {
	if v < 0.0 { return a; }
	if v > 1.0 { return b; }
	return a * (1.0-v) + b * v;
}

