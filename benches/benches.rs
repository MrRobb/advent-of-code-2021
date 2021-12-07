use std::fs::read_to_string;

use advent_of_code_2021::day1::{calculate_increases, calculate_with_sliding_window};
use advent_of_code_2021::day2::{calculate_depth_position, calculate_depth_position_aim};
use advent_of_code_2021::day3::{calculate_oxygen_co2, calculate_power_consumption};
use advent_of_code_2021::day4::{calculate_first_winner, calculate_last_winner};
use advent_of_code_2021::day5::{calculate_all_lines, calculate_horizontal_vertical};
use advent_of_code_2021::day6::calculate_lanternfishes;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench1(c: &mut Criterion) {
	let input1 = read_to_string("input/day1/input.txt").expect("Input file not found");
	let input2 = read_to_string("input/day2/input.txt").expect("Input file not found");
	let input3 = read_to_string("input/day3/input.txt").expect("Input file not found");
	let input4 = read_to_string("input/day4/input.txt").expect("Input file not found");
	let input5 = read_to_string("input/day5/input.txt").expect("Input file not found");
	let input6 = read_to_string("input/day6/input.txt").expect("Input file not found");

	c.bench_function("Day 1 | Part 1", |b| b.iter(|| calculate_increases(&input1)));
	c.bench_function("Day 1 | Part 2", |b| b.iter(|| calculate_with_sliding_window(&input1)));

	c.bench_function("Day 2 | Part 1", |b| b.iter(|| calculate_depth_position(&input2)));
	c.bench_function("Day 2 | Part 2", |b| b.iter(|| calculate_depth_position_aim(&input2)));

	c.bench_function("Day 3 | Part 1", |b| b.iter(|| calculate_power_consumption(&input3)));
	c.bench_function("Day 3 | Part 2", |b| b.iter(|| calculate_oxygen_co2(&input3)));

	c.bench_function("Day 4 | Part 1", |b| b.iter(|| calculate_first_winner(&input4)));
	c.bench_function("Day 4 | Part 2", |b| b.iter(|| calculate_last_winner(&input4)));

	c.bench_function("Day 5 | Part 1", |b| b.iter(|| calculate_horizontal_vertical(&input5)));
	c.bench_function("Day 5 | Part 2", |b| b.iter(|| calculate_all_lines(&input5)));

	c.bench_function("Day 6 | Part 1", |b| b.iter(|| calculate_lanternfishes(&input6, 80)));
	c.bench_function("Day 6 | Part 2", |b| b.iter(|| calculate_lanternfishes(&input6, 256)));
}

// fn bench2(c: &mut Criterion) {
// 	let input = read_to_string("input/day/input.txt").expect("Input file not found");
// 	c.bench_function("Testing function 1", |b| b.iter(|| function1(&input)));
// 	c.bench_function("Testing function 2", |b| b.iter(|| function2(&input)));
// }

criterion_group!(benches, bench1);
criterion_main!(benches);
