#![allow(
	clippy::must_use_candidate,
	clippy::missing_panics_doc,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss
)]

use std::fs::read_to_string;

fn median(numbers: &mut [usize]) -> usize {
	numbers.sort_unstable();
	let mid = numbers.len() / 2;
	numbers[mid]
}

fn average(numbers: &[usize]) -> usize {
	(numbers.iter().sum::<usize>() as f64 / numbers.len() as f64) as usize
}

pub fn calculate_fuel(input: &str) -> usize {
	let mut numbers = input.split(',').flat_map(str::parse::<usize>).collect::<Vec<_>>();
	let median = median(&mut numbers);
	numbers.into_iter().map(|n| n.max(median) - n.min(median)).sum()
}

pub fn calculate_fuel_advanced(input: &str) -> usize {
	let numbers = input.split(',').flat_map(str::parse::<usize>).collect::<Vec<_>>();
	let median = average(&numbers);
	numbers
		.into_iter()
		.map(|n| {
			let dist = n.max(median) - (n.min(median));
			dist * (dist + 1) / 2
		})
		.sum()
}

pub fn main() {
	let input = read_to_string("input/day7/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_fuel(&input));
	println!("PART 2 = {}", calculate_fuel_advanced(&input));
	println!("Execution time: {:?}", now.elapsed());
}
