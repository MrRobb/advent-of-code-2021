#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::fs::read_to_string;

pub fn calculate_depth_position(input: &str) -> usize {
	let mut depth = 0;
	let mut position = 0;
	for line in input.lines() {
		let (instruction, value) = line.split_once(' ').expect("Bad input");
		match instruction {
			"forward" => position += value.parse::<usize>().unwrap(),
			"down" => depth += value.parse::<usize>().unwrap(),
			"up" => depth -= value.parse::<usize>().unwrap(),
			_ => unreachable!(),
		}
	}
	depth * position
}

pub fn calculate_depth_position_aim(input: &str) -> usize {
	let mut depth = 0;
	let mut position = 0;
	let mut aim = 0;
	for line in input.lines() {
		let (instruction, value) = line.split_once(' ').expect("Bad input");
		match instruction {
			"forward" => {
				position += value.parse::<usize>().unwrap();
				depth += aim * value.parse::<usize>().unwrap();
			},
			"down" => aim += value.parse::<usize>().unwrap(),
			"up" => aim -= value.parse::<usize>().unwrap(),
			_ => unreachable!(),
		}
	}
	depth * position
}

pub fn main() {
	let input = read_to_string("input/day2/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_depth_position(&input));
	println!("PART 2 = {}", calculate_depth_position_aim(&input));
	println!("Execution time: {:?}", now.elapsed());
}
