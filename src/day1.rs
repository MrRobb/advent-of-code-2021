use itertools::Itertools;
use std::fs::read_to_string;

pub fn calculate_increases(input: &str) -> usize {
    input
        .lines()
        .flat_map(str::parse::<u64>)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

pub fn calculate_increases_sliding_window(input: &str) -> usize {
    input
        .lines()
        .flat_map(str::parse::<u64>)
        .tuple_windows()
        .filter(|(a, _, _, b)| a < b)
        .count()
}

pub fn main() {
    let input = read_to_string("input/day1/input.txt").expect("Input file not found");
    println!("PART 1 = {}", calculate_increases(&input));
    println!("PART 2 = {}", calculate_increases_sliding_window(&input));
}
