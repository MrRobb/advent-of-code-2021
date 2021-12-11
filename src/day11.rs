#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::fs::read_to_string;

const STEPS: usize = 100;

struct Octopus {
	energy: u32,
	has_flashed: bool,
}

fn update_if_exists(map: &mut Vec<Vec<Octopus>>, row: Option<usize>, col: Option<usize>) {
	if let Some(r) = row {
		if let Some(c) = col {
			if r < map.len() && c < map.len() {
				map[r][c].energy += 1;
			}
		}
	}
}

fn flash_board(map: &mut Vec<Vec<Octopus>>) -> usize {
	let mut flashes = 0;

	for row in 0..map.len() {
		for col in 0..map[row].len() {
			if map[row][col].energy > 9 && !map[row][col].has_flashed {
				// Flash
				flashes += 1;
				map[row][col].has_flashed = true;

				update_if_exists(map, row.checked_sub(1), col.checked_sub(1));
				update_if_exists(map, row.checked_sub(1), Some(col));
				update_if_exists(map, row.checked_sub(1), col.checked_add(1));

				update_if_exists(map, Some(row), col.checked_sub(1));
				update_if_exists(map, Some(row), Some(col));
				update_if_exists(map, Some(row), col.checked_add(1));

				update_if_exists(map, row.checked_add(1), col.checked_sub(1));
				update_if_exists(map, row.checked_add(1), Some(col));
				update_if_exists(map, row.checked_add(1), col.checked_add(1));
			}
		}
	}

	flashes
}

pub fn calculate_flashes(input: &str) -> usize {
	let mut map = input
		.lines()
		.map(|line| {
			line.chars()
				.map(|c| Octopus {
					energy: c.to_digit(10).unwrap(),
					has_flashed: false,
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	let mut flashes = 0;

	for _ in 0..STEPS {
		// Step 1
		map.iter_mut().flatten().for_each(|octo| octo.energy += 1);

		// Step 2
		while {
			let f = flash_board(&mut map);
			flashes += f;
			f != 0
		} {}

		// Step 3
		map.iter_mut().flatten().for_each(|octo| {
			octo.energy = if octo.has_flashed { 0 } else { octo.energy };
			octo.has_flashed = false;
		});
	}

	flashes
}

pub fn calculate_simulatenous_flash(input: &str) -> usize {
	let mut map = input
		.lines()
		.map(|line| {
			line.chars()
				.map(|c| Octopus {
					energy: c.to_digit(10).unwrap(),
					has_flashed: false,
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	let mut step = 1;

	loop {
		// Step 1
		map.iter_mut().flatten().for_each(|octo| octo.energy += 1);

		// Step 2
		while {
			let f = flash_board(&mut map);
			f != 0
		} {}

		if map.iter().flatten().all(|octo| octo.has_flashed) {
			return step;
		}

		// Step 3
		map.iter_mut().flatten().for_each(|octo| {
			octo.energy = if octo.has_flashed { 0 } else { octo.energy };
			octo.has_flashed = false;
		});
		step += 1;
	}
}

pub fn main() {
	let input = read_to_string("input/day11/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_flashes(&input));
	println!("PART 2 = {}", calculate_simulatenous_flash(&input));
	println!("Execution time: {:?}", now.elapsed());
}
