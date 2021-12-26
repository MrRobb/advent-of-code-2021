#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use advent_of_code_2021::{
	day1, day10, day11, day12, day13, day14, day15, day16, day17, day2, day21, day3, day4, day5, day6, day7, day8, day9, day22,
};

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
		day9::main,
		day10::main,
		day11::main,
		day12::main,
		day13::main,
		day14::main,
		day15::main,
		day16::main,
		day17::main,
		day21::main,
		day22::main,
	];

	let now = std::time::Instant::now();

	for (day, main) in mains.iter().enumerate() {
		println!(
			"------------------------------------ DAY {} ------------------------------------",
			day + 1
		);
		main();
		println!();
	}

	println!("------------------------------------  ALL   ------------------------------------");
	println!("Execution time: {:?}\n", now.elapsed());
}
