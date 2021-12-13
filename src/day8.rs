#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::fs::read_to_string;

use rayon::prelude::*;

pub fn calculate_1478(input: &str) -> usize {
	input
		.lines()
		.filter_map(|line| line.split_once(" | "))
		.flat_map(|(_, output)| output.split(' '))
		.filter(|n| n.len() == 2 || n.len() == 4 || n.len() == 3 || n.len() == 7)
		.count()
}

fn decode_output(output: &str, input: &str) -> usize {
	let input = input.split(' ').collect::<Vec<_>>();
	let one = input.iter().find(|d| d.len() == 2).unwrap();
	let four = input.iter().find(|d| d.len() == 4).unwrap();
	output
		.split(' ')
		.map(|w| match w.len() {
			2 => 1,
			3 => 7,
			4 => 4,
			7 => 8,
			5 => {
				let chars_in_one = w.chars().filter(|c| one.contains(*c)).count();
				let chars_in_four = w.chars().filter(|c| four.contains(*c)).count();
				match (chars_in_one, chars_in_four) {
					(1, 3) => 5,
					(2, 3) => 3,
					(_, 2) => 2,
					_ => unreachable!(),
				}
			},
			6 => {
				let chars_in_one = w.chars().filter(|c| one.contains(*c)).count();
				let chars_in_four = w.chars().filter(|c| four.contains(*c)).count();
				match (chars_in_one, chars_in_four) {
					(1, _) => 6,
					(_, 3) => 0,
					(_, 4) => 9,
					_ => unreachable!(),
				}
			},
			_ => unreachable!(),
		})
		.fold(0, |acc, n| acc * 10 + n)
}

pub fn calculate_output(input: &str) -> usize {
	input
		.lines()
		.par_bridge()
		.map(|line| {
			let (input, output) = line.split_once(" | ").unwrap();
			decode_output(output, input)
		})
		.sum::<usize>()
}

pub fn main() {
	let input = read_to_string("input/day8/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_1478(&input));
	println!("PART 2 = {}", calculate_output(&input));
	println!("Execution time: {:?}", now.elapsed());
}
