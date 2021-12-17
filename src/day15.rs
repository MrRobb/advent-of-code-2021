#![allow(
	clippy::must_use_candidate,
	clippy::missing_panics_doc,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss
)]

use std::fs::read_to_string;

use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use pathfinding::prelude::{Matrix, Weights};

fn successors_small_map<'a>(
	map: &'a Matrix<u32>,
	node: &(usize, usize),
) -> impl Iterator<Item = ((usize, usize), u32)> + 'a {
	let neighbours = map.neighbours(*node, false).collect_vec();
	let costs = neighbours
		.clone()
		.into_iter()
		.filter_map(|neighbour| map.get(neighbour))
		.copied();
	neighbours.into_iter().zip(costs)
}

pub fn calculate_path_cost(input: &str) -> u32 {
	let map = input
		.lines()
		.map(|line| line.chars().filter_map(|c| c.to_digit(10)))
		.collect::<Matrix<_>>();

	let (_, cost) = dijkstra(
		&(0, 0),
		|v| successors_small_map(&map, v),
		|(x, y)| *x == map.rows - 1 && *y == map.columns - 1,
	)
	.expect("No path found");

	cost
}

fn successors_big_map(map: &Matrix<u32>, node: (i64, i64)) -> impl Iterator<Item = ((usize, usize), usize)> + '_ {
	let s = map.rows();
	[(1_i64, 0_i64), (-1, 0), (0, 1), (0, -1)]
		.iter()
		.map(move |&(xx, yy)| ((node.0 + xx) as usize, (node.1 + yy) as usize))
		.filter(move |(x, y)| (x / 5 < s && y / 5 < s))
		.filter_map(move |(x, y)| {
			map.get((y % s, x % s))
				.map(|c| ((x, y), ((*c as usize + (x / s) + (y / s) - 1) % 9 + 1)))
		})
}

pub fn calculate_path_cost_5x(input: &str) -> usize {
	let map = input
		.lines()
		.map(|line| line.chars().filter_map(|c| c.to_digit(10)))
		.collect::<Matrix<_>>();

	let (_, cost) = dijkstra(
		&(0, 0),
		|&(x, y)| successors_big_map(&map, (x as i64, y as i64)),
		|(x, y)| *x == 5 * map.rows() - 1 && *y == 5 * map.columns() - 1,
	)
	.expect("No path found");

	cost
}

pub fn main() {
	let input = read_to_string("input/day15/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_path_cost(&input));
	println!("PART 2 = {}", calculate_path_cost_5x(&input));
	println!("Execution time: {:?}", now.elapsed());
}
