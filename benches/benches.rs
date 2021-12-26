use std::fs::read_to_string;

use advent_of_code_2021::day1::{calculate_increases, calculate_with_sliding_window};
use advent_of_code_2021::day10::{calculate_incomplete, calculate_incorrect};
use advent_of_code_2021::day11::{calculate_flashes, calculate_simulatenous_flash};
use advent_of_code_2021::day12::{calculate_paths, calculate_paths_repeating};
use advent_of_code_2021::day13::{calculate_all_folds, calculate_one_fold};
use advent_of_code_2021::day14::calculate_frequencies;
use advent_of_code_2021::day15::{calculate_path_cost, calculate_path_cost_5x};
use advent_of_code_2021::day16::{calculate_eval_expression, calculate_sum_versions};
use advent_of_code_2021::day17::{calculate_distinct, calculate_max_y};
use advent_of_code_2021::day2::{calculate_depth_position, calculate_depth_position_aim};
use advent_of_code_2021::day21::{calculate_quantum, calculate_play};
use advent_of_code_2021::day22::{calculate_how_many_on_coords, calculate_how_many_on};
use advent_of_code_2021::day3::{calculate_oxygen_co2, calculate_power_consumption};
use advent_of_code_2021::day4::{calculate_first_winner, calculate_last_winner};
use advent_of_code_2021::day5::{calculate_all_lines, calculate_horizontal_vertical};
use advent_of_code_2021::day6::calculate_lanternfishes;
use advent_of_code_2021::day7::{calculate_fuel, calculate_fuel_advanced};
use advent_of_code_2021::day8::{calculate_1478, calculate_output};
use advent_of_code_2021::day9::{calculate_basins, calculate_low_points};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench1(c: &mut Criterion) {
	let input01 = read_to_string("input/day1/input.txt").expect("Input file not found");
	let input02 = read_to_string("input/day2/input.txt").expect("Input file not found");
	let input03 = read_to_string("input/day3/input.txt").expect("Input file not found");
	let input04 = read_to_string("input/day4/input.txt").expect("Input file not found");
	let input05 = read_to_string("input/day5/input.txt").expect("Input file not found");
	let input06 = read_to_string("input/day6/input.txt").expect("Input file not found");
	let input07 = read_to_string("input/day7/input.txt").expect("Input file not found");
	let input08 = read_to_string("input/day8/input.txt").expect("Input file not found");
	let input09 = read_to_string("input/day9/input.txt").expect("Input file not found");
	let input10 = read_to_string("input/day10/input.txt").expect("Input file not found");
	let input11 = read_to_string("input/day11/input.txt").expect("Input file not found");
	let input12 = read_to_string("input/day12/input.txt").expect("Input file not found");
	let input13 = read_to_string("input/day13/input.txt").expect("Input file not found");
	let input14 = read_to_string("input/day14/input.txt").expect("Input file not found");
	let input15 = read_to_string("input/day15/input.txt").expect("Input file not found");
	let input16 = read_to_string("input/day16/input.txt").expect("Input file not found");
	let input17 = read_to_string("input/day17/input.txt").expect("Input file not found");
	let input21 = read_to_string("input/day21/input.txt").expect("Input file not found");
	let input22 = read_to_string("input/day22/input.txt").expect("Input file not found");

	c.bench_function("Day 1 | Part 1", |b| b.iter(|| calculate_increases(&input01)));
	c.bench_function("Day 1 | Part 2", |b| b.iter(|| calculate_with_sliding_window(&input01)));

	c.bench_function("Day 2 | Part 1", |b| b.iter(|| calculate_depth_position(&input02)));
	c.bench_function("Day 2 | Part 2", |b| b.iter(|| calculate_depth_position_aim(&input02)));

	c.bench_function("Day 3 | Part 1", |b| b.iter(|| calculate_power_consumption(&input03)));
	c.bench_function("Day 3 | Part 2", |b| b.iter(|| calculate_oxygen_co2(&input03)));

	c.bench_function("Day 4 | Part 1", |b| b.iter(|| calculate_first_winner(&input04)));
	c.bench_function("Day 4 | Part 2", |b| b.iter(|| calculate_last_winner(&input04)));

	c.bench_function("Day 5 | Part 1", |b| b.iter(|| calculate_horizontal_vertical(&input05)));
	c.bench_function("Day 5 | Part 2", |b| b.iter(|| calculate_all_lines(&input05)));

	c.bench_function("Day 6 | Part 1", |b| b.iter(|| calculate_lanternfishes(&input06, 80)));
	c.bench_function("Day 6 | Part 2", |b| b.iter(|| calculate_lanternfishes(&input06, 256)));

	c.bench_function("Day 7 | Part 1", |b| b.iter(|| calculate_fuel(&input07)));
	c.bench_function("Day 7 | Part 2", |b| b.iter(|| calculate_fuel_advanced(&input07)));

	c.bench_function("Day 8 | Part 1", |b| b.iter(|| calculate_1478(&input08)));
	c.bench_function("Day 8 | Part 2", |b| b.iter(|| calculate_output(&input08)));

	c.bench_function("Day 9 | Part 1", |b| b.iter(|| calculate_low_points(&input09)));
	c.bench_function("Day 9 | Part 2", |b| b.iter(|| calculate_basins(&input09)));

	c.bench_function("Day 10 | Part 1", |b| b.iter(|| calculate_incorrect(&input10)));
	c.bench_function("Day 10 | Part 2", |b| b.iter(|| calculate_incomplete(&input10)));

	c.bench_function("Day 11 | Part 1", |b| b.iter(|| calculate_flashes(&input11)));
	c.bench_function("Day 11 | Part 2", |b| b.iter(|| calculate_simulatenous_flash(&input11)));

	c.bench_function("Day 12 | Part 1", |b| b.iter(|| calculate_paths(&input12)));
	c.bench_function("Day 12 | Part 2", |b| b.iter(|| calculate_paths_repeating(&input12)));

	c.bench_function("Day 13 | Part 1", |b| b.iter(|| calculate_one_fold(&input13)));
	c.bench_function("Day 13 | Part 2", |b| b.iter(|| calculate_all_folds(&input13)));

	c.bench_function("Day 14 | Part 1", |b| b.iter(|| calculate_frequencies(&input14, 10)));
	c.bench_function("Day 14 | Part 2", |b| b.iter(|| calculate_frequencies(&input14, 40)));

	c.bench_function("Day 15 | Part 1", |b| b.iter(|| calculate_path_cost(&input15)));
	c.bench_function("Day 15 | Part 2", |b| b.iter(|| calculate_path_cost_5x(&input15)));

	c.bench_function("Day 16 | Part 1", |b| b.iter(|| calculate_sum_versions(&input16)));
	c.bench_function("Day 16 | Part 2", |b| b.iter(|| calculate_eval_expression(&input16)));

	c.bench_function("Day 17 | Part 1", |b| b.iter(|| calculate_max_y(&input17)));
	c.bench_function("Day 17 | Part 2", |b| b.iter(|| calculate_distinct(&input17)));

	c.bench_function("Day 21 | Part 1", |b| b.iter(|| calculate_play(&input21)));
	c.bench_function("Day 21 | Part 2", |b| b.iter(|| calculate_quantum(&input21)));

	c.bench_function("Day 22 | Part 1", |b| b.iter(|| calculate_how_many_on_coords(&input22)));
	c.bench_function("Day 22 | Part 2", |b| b.iter(|| calculate_how_many_on(&input22)));
}

// fn bench2(c: &mut Criterion) {
// 	let input = read_to_string("input/day12/input.txt").expect("Input file not found");
// 	c.bench_function("Testing function 1", |b| {
// 		b.iter(|| calculate_paths_repeating(&input));
// 	});
// }

criterion_group!(benches, bench1);
criterion_main!(benches);
