#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::fs::read_to_string;

use itertools::Itertools;

const WIDTH: usize = 5;

#[derive(Clone)]
struct Number {
	number: u32,
	drawned: bool,
}

struct Bingo {
	numbers: Vec<Number>,
}

impl Bingo {
	fn new(chunks: &[&str]) -> Self {
		let numbers: Vec<Number> = chunks
			.iter()
			.flat_map(|chunk| {
				chunk.split_whitespace().map(|num| Number {
					number: num.parse().unwrap(),
					drawned: false,
				})
			})
			.collect();

		Self { numbers }
	}

	fn drawn_number(&mut self, drawned_number: u32) {
		for n in &mut self.numbers {
			if n.number == drawned_number {
				n.drawned = true;
			}
		}
	}

	fn is_winner(&self) -> bool {
		let row_winner = self.numbers.chunks(WIDTH).any(|chunk| chunk.iter().all(|n| n.drawned));

		let col_winner = (0..WIDTH)
			.flat_map(|col| self.numbers.iter().skip(col).step_by(WIDTH))
			.chunks(WIDTH)
			.into_iter()
			.any(|chunk| chunk.into_iter().all(|n| n.drawned));

		row_winner || col_winner
	}

	fn sum_unmarked(&self) -> u32 {
		self.numbers.iter().filter(|n| !n.drawned).map(|n| n.number).sum()
	}
}

pub fn calculate_first_winner(input: &str) -> u32 {
	let mut lines = input.lines();
	let mut numbers = lines.next().unwrap().split(',').map(|s| s.parse().unwrap());

	let mut bingos = Vec::new();
	for chunk in &lines.filter(|&l| !l.is_empty()).chunks(WIDTH) {
		bingos.push(Bingo::new(&chunk.collect::<Vec<_>>()));
	}

	numbers
		.find_map(|number| {
			bingos
				.iter_mut()
				.map(|bingo| {
					bingo.drawn_number(number);
					bingo
				})
				.filter(|b| b.is_winner())
				.map(|winner_b| winner_b.sum_unmarked() * number)
				.next()
		})
		.unwrap()
}

pub fn calculate_last_winner(input: &str) -> u32 {
	let mut lines = input.lines();
	let numbers = lines.next().unwrap().split(',').map(|s| s.parse().unwrap());

	let mut bingos = Vec::new();
	for chunk in &lines.filter(|&l| !l.is_empty()).chunks(WIDTH) {
		bingos.push(Bingo::new(&chunk.collect::<Vec<_>>()));
	}

	numbers
		.filter_map(|number| {
			bingos
				.iter_mut()
				.filter(|b| !b.is_winner())
				.map(|bingo| {
					bingo.drawn_number(number);
					bingo
				})
				.filter(|b| b.is_winner())
				.map(|winner_b| winner_b.sum_unmarked() * number)
				.last()
		})
		.last()
		.unwrap()
}

pub fn main() {
	let input = read_to_string("input/day4/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_first_winner(&input));
	println!("PART 2 = {}", calculate_last_winner(&input));
	println!("Execution time: {:?}", now.elapsed());
}
