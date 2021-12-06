use advent_of_code_2021::{day1, day2, day3, day4};

fn main() {
	let mains = [day1::main, day2::main, day3::main, day4::main];

	for (day, main) in mains.iter().enumerate() {
		println!(
			"------------------------------------ DAY {} ------------------------------------",
			day + 1
		);
		main();
		println!();
	}
}
