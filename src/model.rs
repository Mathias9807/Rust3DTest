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
		Model{verts: Vec::<Vertex>::new(),
			faces: Vec::<(usize, usize, usize)>::new()}
	}

	pub fn load_obj(file: &str) -> Model {
		let mut model = Model::new();
	
		let obj = fs::read_to_string(file).expect("Couldn't read the file");
	
		let re = Regex::new(r"^(.?) ([-.0-9]+) ([-.0-9]+) ([-.0-9]+)$").unwrap();
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
				model.verts.push(Vertex::vertex(
					i0.parse().unwrap(),
					i1.parse().unwrap(),
					i2.parse().unwrap(),
				));
			}
			if t == "f" {
				model.faces.push((
					i0.parse::<usize>().unwrap() - 1,
					i1.parse::<usize>().unwrap() - 1,
					i2.parse::<usize>().unwrap() - 1,
				));
			}
		}
	
		return model;
	}

}

