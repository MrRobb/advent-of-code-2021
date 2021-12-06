use std::fs::read_to_string;

use itertools::Itertools;

pub fn calculate_increases(input: &str) -> usize {
	input
		.lines()
		.flat_map(str::parse::<u64>)
		.tuple_windows()
		.filter(|(a, b)| a < b)
		.count()
}

pub fn calculate_with_sliding_window(input: &str) -> usize {
	input
		.lines()
		.flat_map(str::parse::<u64>)
		.tuple_windows()
		.filter(|(a, _, _, b)| a < b)
		.count()
}

pub fn main() {
	let input = read_to_string("input/day1/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_increases(&input));
	println!("PART 2 = {}", calculate_with_sliding_window(&input));
	println!("Execution time: {:?}", now.elapsed())
}
