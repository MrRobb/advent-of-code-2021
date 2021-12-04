
use std::fs::read_to_string;
use itertools::Itertools;

fn calculate_increases(input: &str) -> usize {
    input.lines()
        .flat_map(str::parse::<u64>)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

pub fn main() {
    let input = read_to_string("input/day1/input1.txt")
        .expect("Input file not found");
    println!("RESULT = {}", calculate_increases(&input));
}