use std::fs::read_to_string;

use scan_fmt::scan_fmt;

const MAP_SIZE: usize = 1000;

fn generate_path(x1: usize, y1: usize, x2: usize, y2: usize) -> Vec<(usize, usize)> {
	// Vertical
	if x1 == x2 {
		(y1.min(y2)..=y2.max(y1)).map(|y| (x1, y)).collect()
	}
	// Horizontal
	else if y1 == y2 {
		(x1.min(x2)..=x2.max(x1)).map(|x| (x, y1)).collect()
	}
	// Diagonal
	else {
		if x1 <= x2 {
			if y1 <= y2 {
				(0..=(x2 - x1)).map(|inc| (x1 + inc, y1 + inc)).collect()
			}
			else {
				(0..=(x2 - x1)).map(|inc| (x1 + inc, y1 - inc)).collect()
			}
		}
		else {
			if y1 <= y2 {
				(0..=(x1 - x2)).map(|inc| (x1 - inc, y1 + inc)).collect()
			}
			else {
				(0..=(x1 - x2)).map(|inc| (x1 - inc, y1 - inc)).collect()
			}
		}
	}
}

pub fn calculate_horizontal_vertical(input: &str) -> usize {
	let mut map = vec![0; MAP_SIZE * MAP_SIZE];
	input
		.lines()
		.map(|line| {
			let (x1, y1, x2, y2) = scan_fmt!(line, "{d},{d} -> {d},{d}", usize, usize, usize, usize).unwrap();
			(x1, y1, x2, y2)
		})
		.filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
		.for_each(|(x1, y1, x2, y2)| {
			generate_path(x1, y1, x2, y2)
				.into_iter()
				.for_each(|(x, y)| map[y * MAP_SIZE + x] += 1)
		});
	map.into_iter().filter(|&overlaps| overlaps >= 2).count()
}

pub fn calculate_horizontal_vertical_diagonal(input: &str) -> usize {
	let mut map = vec![0; MAP_SIZE * MAP_SIZE];

	input
		.lines()
		.map(|line| {
			let (x1, y1, x2, y2) = scan_fmt!(line, "{d},{d} -> {d},{d}", usize, usize, usize, usize).unwrap();
			(x1, y1, x2, y2)
		})
		.for_each(|(x1, y1, x2, y2)| {
			generate_path(x1, y1, x2, y2)
				.into_iter()
				.for_each(|(x, y)| map[y * MAP_SIZE + x] += 1)
		});
	map.into_iter().filter(|&overlaps| overlaps >= 2).count()
}

pub fn main() {
	let input = read_to_string("input/day5/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_horizontal_vertical(&input));
	println!("PART 2 = {}", calculate_horizontal_vertical_diagonal(&input));
	println!("Execution time: {:?}", now.elapsed());
}
