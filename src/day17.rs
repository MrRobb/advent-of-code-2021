#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::fs::read_to_string;

use itertools::Itertools;

struct Area {
	x_min: i64,
	x_max: i64,
	y_min: i64,
	y_max: i64,
}

const fn find_max_y(area: &Area) -> i64 {
	(area.y_min * -!area.y_min) >> 1
}

const fn try_trajectory(mut x_vel: i64, mut y_vel: i64, area: &Area) -> bool {
	let mut x = 0;
	let mut y = 0;
	loop {
		match (area.x_min <= x && x <= area.x_max, area.y_min <= y && y <= area.y_max) {
			(true, true) => return true,
			(false, _) if x_vel == 0 => return false,
			(_, false) if y_vel < 0 && y <= area.y_min => return false,
			_ => {},
		}
		x += x_vel;
		y += y_vel;
		x_vel -= x_vel.signum();
		y_vel -= 1;
	}
}

fn find_distinct(area: &Area) -> usize {
	(0..=area.x_max.max(0))
		.cartesian_product(area.y_min..=area.y_max.abs() * 2)
		.filter(|(x_vel, y_vel)| try_trajectory(*x_vel, *y_vel, area))
		.unique()
		.count()
}

pub fn calculate_max_y(input: &str) -> i64 {
	let (_, b) = input.split_once("target area: x=").unwrap();
	let (x, y) = b.split_once(", y=").unwrap();
	let (x_min_str, x_max_str) = x.split_once("..").unwrap();
	let (x_min, x_max) = (x_min_str.parse().unwrap(), x_max_str.parse().unwrap());
	let (y_min_str, y_max_str) = y.split_once("..").unwrap();
	let (y_min, y_max) = (y_min_str.parse().unwrap(), y_max_str.parse().unwrap());

	find_max_y(&Area {
		x_min,
		x_max,
		y_min,
		y_max,
	})
}

pub fn calculate_distinct(input: &str) -> usize {
	let (_, b) = input.split_once("target area: x=").unwrap();
	let (x, y) = b.split_once(", y=").unwrap();
	let (x_min_str, x_max_str) = x.split_once("..").unwrap();
	let (x_min, x_max) = (x_min_str.parse().unwrap(), x_max_str.parse().unwrap());
	let (y_min_str, y_max_str) = y.split_once("..").unwrap();
	let (y_min, y_max) = (y_min_str.parse().unwrap(), y_max_str.parse().unwrap());

	find_distinct(&Area {
		x_min,
		x_max,
		y_min,
		y_max,
	})
}

pub fn main() {
	let input = read_to_string("input/day17/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_max_y(&input));
	println!("PART 2 = {}", calculate_distinct(&input));
	println!("Execution time: {:?}", now.elapsed());
}
