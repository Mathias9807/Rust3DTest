#![allow(dead_code, unused_variables, non_snake_case)]

use std::ops;

use crate::{WIDTH, HEIGHT};

pub const MIN_SCREEN: Vec2f = Vec2f([-5.0, -5.0]);
pub const MAX_SCREEN: Vec2f = Vec2f([ 5.0,  5.0]);

pub fn toDCoords(p: Vec2f) -> Vec2f {
	(p - MIN_SCREEN) / (MAX_SCREEN - MIN_SCREEN)
	* Vec2f([WIDTH as f32, HEIGHT as f32])
}

pub fn lerp(a: f32, b: f32, v: f32) -> f32 {
	if v < 0.0 { return a; }
	if v > 1.0 { return b; }
	return a * (1.0-v) + b * v;
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
	pub p: Vec3f,
	pub c: Vec3f
}
impl Vertex {
	pub fn new(x: f32, y: f32, z: f32) -> Vertex {
		Vertex{p: Vec3f([x, y, z]), c: Vec3f([1.0, 1.0, 1.0])}
	}
	pub fn color(&mut self, r: f32, g: f32, b: f32) -> &Vertex {
		self.c = Vec3f([r, g, b]); self
	}
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Mat4([[f32; 4]; 4]);
impl Mat4 {
	pub const fn identity() -> Mat4 {
		Mat4([
            [1.,0.,0.,0.],
            [0.,1.,0.,0.],
            [0.,0.,1.,0.],
            [0.,0.,0.,1.]])
	}

    pub fn rotate(mut self, axis: Vec3f, angle: f32) -> Mat4 {
        let s = angle.sin();
        let c = angle.cos();

        let mut u = axis;
        u.normalize();
        let mut T = Mat4::identity();
        for i in 0..4 { for j in 0..4 {
            T[i][j] = if i<3 && j<3 { u[i] * u[j] } else { 0. };
        }}

        let S = Mat4([
            [   0.,  u[2], -u[1], 0.],
            [-u[2],    0.,  u[0], 0.],
            [ u[1], -u[0],    0., 0.],
            [   0.,    0.,    0., 0.]
        ]) * s;

        let C = (Mat4::identity() - T) * c;

        T = T + C + S;

        T[3][3] = 1.;		
        self = self * T; self
    }

    pub fn translate3f(mut self, x:f32, y:f32, z:f32) -> Mat4 {
        self[3][0] += x; self[3][1] += y; self[3][2] += z; self
    }
}
impl ops::Index<usize> for Mat4 {
	type Output = [f32; 4];
	fn index(&self, i: usize) -> &[f32; 4] { &self.0[i] }
}
impl ops::IndexMut<usize> for Mat4 {
	fn index_mut(&mut self, i: usize) -> &mut [f32; 4] { &mut self.0[i] }
}
macro_rules! mat_operator { ($op_name:ident, $op_fn:ident) => {
        impl ops::$op_name<Mat4> for Mat4 {
            type Output = Mat4;
            fn $op_fn(self, rhs: Mat4) -> Mat4 {
                let mut r = Mat4::default();
                for i in 0..4 { for j in 0..4 {
                    r[i][j] = self[i][j].$op_fn(rhs[i][j]);
                }} r
            }
        }
} }
macro_rules! mat_operator_scalar { ($op_name:ident, $op_fn:ident) => {
        impl ops::$op_name<f32> for Mat4 {
            type Output = Mat4;
            fn $op_fn(self, rhs: f32) -> Mat4 {
                let mut r = Mat4::default();
                for j in 0..4 { for i in 0..4 {
                    r[i][j] = self[i][j].$op_fn(rhs);
                }} r
            }
        }
} }
mat_operator!(Add, add);
mat_operator!(Sub, sub);
mat_operator_scalar!(Add, add);
mat_operator_scalar!(Sub, sub);
mat_operator_scalar!(Mul, mul);
mat_operator_scalar!(Div, div);
impl ops::Mul<Vec4f> for Mat4 {
    type Output = Vec4f;
    fn mul(self, rhs: Vec4f) -> Vec4f {
        let mut r = Vec4f::new([0.,0.,0.,0.]);
        for j in 0..4 {
            for i in 0..4 { r[j] += self[i][j] * rhs[i]; }
        } r
    }
}
impl ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        let mut temp = Mat4::default();
        for c in 0..4 { for r in 0..4 {
            for k in 0..4 { temp[c][r] += self[k][r] * rhs[c][k]; }
        }} temp
    }
}

macro_rules! vec_operator {
	($type_name:ident, $size:expr, $op_name:ident, $op_fn:ident) => {
		impl ops::$op_name<$type_name> for $type_name {
			type Output = $type_name;
			fn $op_fn(self, rhs: $type_name) -> $type_name {
				let mut r = self;
				for i in 0..$size { r[i] = self[i].$op_fn(rhs[i]); } r }
		}
	}
}
macro_rules! vec_operator_scalar {
	($type_name:ident, $size:expr, $type:ty, $op_name:ident, $op_fn:ident) => {
		impl ops::$op_name<$type> for $type_name {
			type Output = $type_name;
			fn $op_fn(self, rhs: $type) -> $type_name {
				let mut r = self;
				for i in 0..$size { r[i] = self[i].$op_fn(rhs); } r }
		}
	}
}
macro_rules! declare_vec {
	($type_name:ident, $size:expr, $type:ty) => {
		#[derive(Copy, Clone, Debug)]
		pub struct $type_name(pub [$type; $size]);

		impl $type_name {
			pub fn new(v: [$type; $size]) -> $type_name { $type_name(v) }

            pub fn length(&self) -> f32 {
                let mut l = 0.;
                for i in 0..$size { l += self[i]*self[i]; } l.sqrt()
            }
            pub fn normalize(&mut self) -> &mut $type_name {
                *self = *self / self.length(); self
            }
		}

		impl ops::Index<usize> for $type_name {
			type Output = $type;
			fn index(&self, i: usize) -> &$type { &self.0[i] }
		}
		impl ops::IndexMut<usize> for $type_name {
			fn index_mut(&mut self, i: usize) -> &mut $type { &mut self.0[i] }
		}

		vec_operator!($type_name, $size, Add, add);
		vec_operator_scalar!($type_name, $size, $type, Add, add);
		vec_operator!($type_name, $size, Sub, sub);
		vec_operator_scalar!($type_name, $size, $type, Sub, sub);
		vec_operator!($type_name, $size, Mul, mul);
		vec_operator_scalar!($type_name, $size, $type, Mul, mul);
		vec_operator!($type_name, $size, Div, div);
		vec_operator_scalar!($type_name, $size, $type, Div, div);
	}
}
declare_vec!(Vec2f, 2, f32);
declare_vec!(Vec3f, 3, f32);
declare_vec!(Vec4f, 4, f32);
impl Vec2f {
	pub fn lerp(a: Vec2f, b: Vec2f, v: f32) -> Vec2f {
		Vec2f([lerp(a[0],b[0],v), lerp(a[1],b[1],v)])
	}
}
impl Vec3f {
	pub fn lerp(a: Vec3f, b: Vec3f, v: f32) -> Vec3f {
		Vec3f([lerp(a[0],b[0],v), lerp(a[1],b[1],v), lerp(a[2],b[2],v)])
	}
    pub fn cross(&self, v: Vec3f) -> Vec3f {
        Vec3f([self[1]*v[2] - self[2]*v[1],
              self[2]*v[0] - self[0]*v[2],
              self[0]*v[1] - self[1]*v[0]])
    }
	pub fn vec2f(&self) -> Vec2f { Vec2f([self[0], self[1]]) }
	pub fn vec4f(&self, a: f32) -> Vec4f { Vec4f([self.0[0], self.0[1], self.0[2], a]) }
}
impl Vec4f {
	pub fn vec2f(&self) -> Vec2f { Vec2f([self[0], self[1]]) }
	pub fn vec3f(&self) -> Vec3f { Vec3f([self[0], self[1], self[2]]) }
}

pub fn VecTest() {
	let m = Mat4::identity().rotate(Vec3f([0., 1., 0.]), 1.);
	println!("{:?}", m);

    let n = 5.0;
	println!("{:?}\n * {}\n = {:?}", m, n, m * n);

    let mut w = Vec4f::new([5., 4., 3., 0.]);
	println!("Length = {}", w.length());
	println!("Normalized = {:?}", w.normalize());
}

