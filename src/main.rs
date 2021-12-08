#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use advent_of_code_2021::{day1, day2, day3, day4, day5, day6, day7, day8};

fn main() {
	let mains = [
		day1::main,
		day2::main,
		day3::main,
		day4::main,
		day5::main,
		day6::main,
		day7::main,
		day8::main,
	];

	for (day, main) in mains.iter().enumerate() {
		println!(
			"------------------------------------ DAY {} ------------------------------------",
			day + 1
		);
		main();
		println!();
	}
}
