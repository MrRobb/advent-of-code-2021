#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::fs::read_to_string;

fn median(numbers: &mut [usize]) -> usize {
	numbers.sort_unstable();
	let mid = numbers.len() / 2;
	numbers[mid]
}

fn match_brackets(c: char) -> char {
	match c {
		'(' => ')',
		'[' => ']',
		'{' => '}',
		'<' => '>',
		')' => '(',
		']' => '[',
		'}' => '{',
		'>' => '<',
		_ => unreachable!(),
	}
}

pub fn calculate_incorrect(input: &str) -> usize {
	input
		.lines()
		.map(|line| {
			let mut stack = Vec::new();
			let mut incorrect = None;
			line.chars().try_fold(&mut stack, |stack, c| match c {
				'(' | '[' | '{' | '<' => {
					stack.push(c);
					Some(stack)
				},
				')' | ']' | '}' | '>' => {
					if *stack.last().unwrap() == match_brackets(c) {
						stack.pop();
						Some(stack)
					}
					else {
						incorrect = Some(c);
						None
					}
				},
				_ => unreachable!(),
			});

			// Correct and complete
			if stack.is_empty() {
				0
			}
			else {
				match incorrect {
					// Incorrect
					Some(')') => 3,
					Some(']') => 57,
					Some('}') => 1197,
					Some('>') => 25137,
					Some(_) => unreachable!(),

					// Correct and incomplete
					None => 0,
				}
			}
		})
		.sum()
}

pub fn calculate_incomplete(input: &str) -> usize {
	let mut scores = input
		.lines()
		.map(|line| {
			let mut stack = Vec::new();
			let mut incorrect = None;
			line.chars().try_fold(&mut stack, |stack, c| match c {
				'(' | '[' | '{' | '<' => {
					stack.push(c);
					Some(stack)
				},
				')' | ']' | '}' | '>' => {
					if *stack.last().unwrap() == match_brackets(c) {
						stack.pop();
						Some(stack)
					}
					else {
						incorrect = Some(c);
						None
					}
				},
				_ => unreachable!(),
			});

			// Correct and complete
			if stack.is_empty() {
				0
			}
			else {
				match incorrect {
					// Incorrect
					Some(_) => 0,

					// Correct and incomplete
					None => stack.iter().rev().fold(0, |score, c| {
						5 * score
							+ match c {
								'(' => 1,
								'[' => 2,
								'{' => 3,
								'<' => 4,
								_ => unreachable!(),
							}
					}),
				}
			}
		})
		.filter(|score| *score != 0)
		.collect::<Vec<_>>();
	median(&mut scores)
}

pub fn main() {
	let input = read_to_string("input/day10/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_incorrect(&input));
	println!("PART 2 = {}", calculate_incomplete(&input));
	println!("Execution time: {:?}", now.elapsed());
}
