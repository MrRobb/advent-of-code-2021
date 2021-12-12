#![allow(clippy::must_use_candidate, clippy::missing_panics_doc, clippy::use_self)]

use std::collections::BTreeMap;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Cave {
	Start,
	End,
	Small(String),
	Big(String),
}

impl From<&str> for Cave {
	fn from(cave: &str) -> Cave {
		match cave {
			"start" => Cave::Start,
			"end" => Cave::End,
			small if small == small.to_lowercase() => Cave::Small(small.into()),
			big if big == big.to_uppercase() => Cave::Big(big.into()),
			_ => unreachable!(),
		}
	}
}

fn find_paths_to_end<'a>(map: &'a BTreeMap<Cave, Vec<Cave>>, from: &'a Cave, visited: &[&'a Cave]) -> usize {
	match from {
		Cave::End => 1,
		cave => {
			if visited.contains(&cave) {
				if let Cave::Small(_) = cave {
					return 0;
				}
				if let Cave::Start = cave {
					return 0;
				}
			}

			map.get(cave).map_or(0, |edges| {
				edges
					.iter()
					.map(|to| find_paths_to_end(map, to, &[visited, &[cave]].concat()))
					.sum()
			})
		},
	}
}

pub fn calculate_paths(input: &str) -> usize {
	let map = input
		.lines()
		.filter_map(|line| line.split_once('-'))
		.fold(BTreeMap::new(), |mut map, (from, to)| {
			let from = Cave::from(from);
			let to = Cave::from(to);
			map.entry(from.clone()).or_insert_with(Vec::new).push(to.clone());
			map.entry(to).or_insert_with(Vec::new).push(from);
			map
		});

	find_paths_to_end(&map, &Cave::Start, &[])
}

fn find_paths_to_end_repeating_once<'a>(
	map: &'a BTreeMap<Cave, Vec<Cave>>,
	from: &'a Cave,
	visited: &[&'a Cave],
	mut repeated: Option<&'a Cave>,
) -> usize {
	match from {
		Cave::End => 1,
		cave => {
			if visited.contains(&cave) {
				if let Cave::Small(_) = cave {
					match repeated {
						None => repeated = Some(cave),
						Some(_) => return 0,
					}
				}
				if let Cave::Start = cave {
					return 0;
				}
			}

			map.get(cave).map_or(0, |edges| {
				edges
					.iter()
					.map(|to| find_paths_to_end_repeating_once(map, to, &[visited, &[cave]].concat(), repeated))
					.sum()
			})
		},
	}
}

pub fn calculate_paths_repeating_once(input: &str) -> usize {
	let map = input
		.lines()
		.filter_map(|line| line.split_once('-'))
		.fold(BTreeMap::new(), |mut map, (from, to)| {
			let from = Cave::from(from);
			let to = Cave::from(to);
			map.entry(from.clone()).or_insert_with(Vec::new).push(to.clone());
			map.entry(to).or_insert_with(Vec::new).push(from);
			map
		});

	find_paths_to_end_repeating_once(&map, &Cave::Start, &[], None)
}

pub fn main() {
	let input = read_to_string("input/day12/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_paths(&input));
	println!("PART 2 = {}", calculate_paths_repeating_once(&input));
	println!("Execution time: {:?}", now.elapsed());
}
