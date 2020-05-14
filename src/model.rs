#![allow(dead_code, unused_variables, non_snake_case)]

extern crate regex;

use std::fs;
use regex::Regex;
use crate::point::*;

#[derive(Clone, Debug)]
pub struct Model {
	pub verts: Vec<Vertex>,
	pub faces: Vec<(usize, usize, usize)>
}
impl Model {
	pub fn new() -> Model {
		Model{verts: Vec::new(),
			faces: Vec::new()}
	}

	pub fn load_obj(file: &str) -> Model {
		let mut model = Model::new();
	
		let obj = fs::read_to_string(file).expect("Couldn't read the file");
	
		let re = Regex::new(r"^(.?) ([-.0-9/]+) ([-.0-9/]+) ([-.0-9/]+)$").unwrap();
		for line in obj.lines() {
			let captures;
			match re.captures(line) {
				Some(c) => captures = c,
				None => continue,
			}
			let t = captures.get(1).unwrap().as_str();
			let i0 = captures.get(2).unwrap().as_str();
			let i1 = captures.get(3).unwrap().as_str();
			let i2 = captures.get(4).unwrap().as_str();

			if t == "v" {
				model.verts.push(Vertex::new(
					i0.parse().unwrap(),
					i1.parse().unwrap(),
					i2.parse().unwrap(),
				));
			}
			if t == "f" {
                let mut splat0 = i0.split('/');
                let mut splat1 = i1.split('/');
                let mut splat2 = i2.split('/');
				model.faces.push((
					splat0.next().unwrap().parse::<usize>().unwrap() - 1,
					splat1.next().unwrap().parse::<usize>().unwrap() - 1,
					splat2.next().unwrap().parse::<usize>().unwrap() - 1,
				));
			}
		}
	
		return model;
	}

}

