#![allow(dead_code, unused_variables, non_snake_case)]

/**
 * Graphics state module, stores depth buffers and various rendering settings
 */

use crate::{WIDTH, HEIGHT};
use crate::point::*;
use crate::model::*;

pub type Buffer8i = [u8; (WIDTH * HEIGHT) as usize];
pub type Buffer3x8i = [(u8, u8, u8); (WIDTH * HEIGHT) as usize];

pub struct GraphicsState<'a> {
    pub d: &'a mut Buffer3x8i
}

impl GraphicsState<'_> {

    /** Draw a model including model-view-projection matrices */
    pub fn draw_model(&mut self, model: &Model, mat: Mat4) {
        let mut proj_vert = model.verts.clone();

        for mut v in &mut proj_vert {
            v.p = (mat * v.p.vec4f(1.0)).vec3f();
        }

        for f in &model.faces {
            self.draw_tri(proj_vert[f.0], proj_vert[f.1], proj_vert[f.2]);
        }
    }

    /** Draw a horizontal line at row y between points a and b */
    pub fn draw_scanline(&mut self, a: f32, b: f32, y: u32, c1: Vec3f, c2: Vec3f) {
        let (mut min, mut max): (i32, i32);
        if a > b { min = b.round() as i32; max = a.round() as i32; }
        else { min = a.round() as i32; max = b.round() as i32; }
        if y >= HEIGHT { return }
        min = min.max(0);
        max = max.min(WIDTH as i32 - 1);
        for x in min..max {
            let inter = (x as f32 - a) / (b - a);
            let c = Vec3f::lerp(c1, c2, inter);
            self.d[(y * WIDTH + x as u32) as usize].0 = (c[0]*255.0) as u8;
            self.d[(y * WIDTH + x as u32) as usize].1 = (c[1]*255.0) as u8;
            self.d[(y * WIDTH + x as u32) as usize].2 = (c[2]*255.0) as u8;
        }
    }

    /** Rasterize triangle a,b,c */
    pub fn draw_tri(&mut self, a: Vertex, b: Vertex, c: Vertex) {
        // Check vertex order, cull if clockwise
        if (b.p - a.p).cross(c.p - a.p)[2] < 0. { return }

        let mut list = [toDCoords(a.p.vec2f()), toDCoords(b.p.vec2f()),
            toDCoords(c.p.vec2f())];
        if list[0][1] > list[1][1] { list.swap(0, 1); } // Bubblesort lol
        if list[0][1] > list[2][1] { list.swap(0, 2); }
        if list[1][1] > list[2][1] { list.swap(1, 2); }

        let midV = (list[1][1] - list[0][1]) / (list[2][1] - list[0][1]);
        let mid = lerp(list[0][0] as f32, list[2][0] as f32, midV);

        for y in list[0][1].round().max(0.0) as u32..list[1][1].round() as u32 {
            let v_a = (y as f32 - list[0][1]) / (list[2][1] - list[0][1]);
            let v_b = (y as f32 - list[0][1]) / (list[1][1] - list[0][1]);
            let a = lerp(list[0][0] as f32, list[2][0] as f32, v_a);
            let b = lerp(list[0][0] as f32, list[1][0], v_b);
            self.draw_scanline(a, b, y, Vec3f([0.0,0.0,0.0]), Vec3f([1.0,1.0,1.0]));
        }
        for y in list[1][1].round().max(0.0) as u32..list[2][1].round() as u32 {
            let v_a = (y as f32 - list[1][1]) / (list[2][1] - list[1][1]);
            let v_b = (y as f32 - list[1][1]) / (list[2][1] - list[1][1]);
            let a = lerp(list[1][0] as f32, list[2][0] as f32, v_a);
            let b = lerp(mid, list[2][0] as f32, v_b);
            self.draw_scanline(a, b, y, Vec3f([0.0,0.0,0.0]), Vec3f([1.0,1.0,1.0]));
        }
    }
}

