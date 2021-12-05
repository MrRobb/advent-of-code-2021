use std::fs::read_to_string;
use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code_2021::day1;
use advent_of_code_2021::day2;
use advent_of_code_2021::day3;

fn criterion_benchmark(c: &mut Criterion) {
    let input1 = read_to_string("input/day1/input.txt").expect("Input file not found");
    let input2 = read_to_string("input/day2/input.txt").expect("Input file not found");
    let input3 = read_to_string("input/day3/input.txt").expect("Input file not found");

    c.bench_function("Day 1 | Part 1", |b| { b.iter(|| day1::calculate_increases(&input1)) });
    c.bench_function("Day 1 | Part 2", |b| { b.iter(|| day1::calculate_increases_sliding_window(&input1)) });

    c.bench_function("Day 2 | Part 1", |b| { b.iter(|| day2::calculate_depth_and_position(&input2)) });
    c.bench_function("Day 2 | Part 2", |b| { b.iter(|| day2::calculate_depth_and_position_and_aim(&input2)) });

    c.bench_function("Day 3 | Part 1", |b| { b.iter(|| day3::calculate_power_consumption(&input3)) });
    c.bench_function("Day 3 | Part 2", |b| { b.iter(|| day3::filter_power_consumption(&input3)) });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
