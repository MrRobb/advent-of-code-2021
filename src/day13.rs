#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::fmt::Write as FmtWrite;
use std::fs::read_to_string;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Axis {
	X,
	Y,
}

impl From<&str> for Axis {
	fn from(axis: &str) -> Self {
		match axis {
			"x" => Self::X,
			"y" => Self::Y,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug)]
struct Fold {
	axis: Axis,
	coordinate: usize,
}

impl Fold {
	fn fold(&self, map: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
		match self.axis {
			Axis::X => map
				.into_iter()
				.filter_map(|(x, y)| match x.cmp(&self.coordinate) {
					std::cmp::Ordering::Less => Some((x, y)),
					std::cmp::Ordering::Equal => None,
					std::cmp::Ordering::Greater => Some((self.coordinate - (x - self.coordinate), y)),
				})
				.collect(),
			Axis::Y => map
				.into_iter()
				.filter_map(|(x, y)| match y.cmp(&self.coordinate) {
					std::cmp::Ordering::Less => Some((x, y)),
					std::cmp::Ordering::Equal => None,
					std::cmp::Ordering::Greater => Some((x, self.coordinate - (y - self.coordinate))),
				})
				.collect(),
		}
	}
}

fn print_map(map: &[(usize, usize)]) -> String {
	let width = map.iter().max_by_key(|(x, _)| x).unwrap().0;
	let height = map.iter().max_by_key(|(_, y)| y).unwrap().1;
	let mut code = String::new();
	for i in 0..=height {
		for j in 0..=width {
			if map.contains(&(j, i)) {
				write!(&mut code, "#").unwrap();
			}
			else {
				write!(&mut code, " ").unwrap();
			}
		}
		writeln!(&mut code).unwrap();
	}
	code
}

pub fn calculate_one_fold(input: &str) -> usize {
	let re = Regex::new("fold along (?P<axis>[xy])=(?P<coordinate>\\d+)").unwrap();

	let map = input
		.lines()
		.map_while(|line| line.split_once(','))
		.map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
		.collect::<Vec<(usize, usize)>>();

	let folds = input
		.lines()
		.skip_while(|line| line.split_once(',').is_some())
		.filter_map(|line| re.captures(line))
		.map(|captures| Fold {
			axis: Axis::from(&captures["axis"]),
			coordinate: captures["coordinate"].parse().unwrap(),
		})
		.collect::<Vec<_>>();

	folds
		.iter()
		.take(1)
		.fold(map, |map, fold| fold.fold(map))
		.iter()
		.unique()
		.count()
}

pub fn calculate_all_folds(input: &str) -> String {
	let re = Regex::new("fold along (?P<axis>[xy])=(?P<coordinate>\\d+)").unwrap();

	let map = input
		.lines()
		.map_while(|line| line.split_once(','))
		.map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
		.collect::<Vec<(usize, usize)>>();

	let folds = input
		.lines()
		.skip_while(|line| line.split_once(',').is_some())
		.filter_map(|line| re.captures(line))
		.map(|captures| Fold {
			axis: Axis::from(&captures["axis"]),
			coordinate: captures["coordinate"].parse().unwrap(),
		})
		.collect::<Vec<_>>();

	print_map(&folds.iter().fold(map, |map, fold| fold.fold(map)))
}

pub fn main() {
	let input = read_to_string("input/day13/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_one_fold(&input));
	println!("PART 2 = \n{}", calculate_all_folds(&input));
	println!("Execution time: {:?}", now.elapsed());
}
