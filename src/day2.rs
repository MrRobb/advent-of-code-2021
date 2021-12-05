use std::fs::read_to_string;

fn calculate_depth_and_position(input: &str) -> usize {
    let mut depth = 0;
    let mut position = 0;
    for line in input.lines() {
        let (instruction, value) = line.split_once(' ').expect("Bad input");
        match instruction {
            "forward" => position += value.parse::<usize>().unwrap(),
            "down" => depth += value.parse::<usize>().unwrap(),
            "up" => depth -= value.parse::<usize>().unwrap(),
            _ => unreachable!(),
        }
    }
    depth * position
}

pub fn calculate_depth_and_position_and_aim(input: &str) -> usize {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    for line in input.lines() {
        let (instruction, value) = line.split_once(' ').expect("Bad input");
        match instruction {
            "forward" => {
                position += value.parse::<usize>().unwrap();
                depth += aim * value.parse::<usize>().unwrap();
            }
            "down" => aim += value.parse::<usize>().unwrap(),
            "up" => aim -= value.parse::<usize>().unwrap(),
            _ => unreachable!(),
        }
    }
    depth * position
}

pub fn main() {
    let input = read_to_string("input/day2/input.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", calculate_depth_and_position(&input));
    println!("PART 2 = {}", calculate_depth_and_position_and_aim(&input));
    println!("Execution time: {:?}", now.elapsed())
}
