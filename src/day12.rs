#![allow(
	clippy::must_use_candidate,
	clippy::missing_panics_doc,
	clippy::use_self,
	clippy::cast_possible_truncation
)]

use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

use rayon::prelude::*;

type DFSFunc<'a> = fn(&'a BTreeMap<Cave, Vec<Cave>>, &'a BTreeMap<Cave, u32>, &'a Cave, usize, bool) -> usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Cave {
	Start,
	End,
	Small(u64),
	Big(u64),
}

impl From<&str> for Cave {
	fn from(cave: &str) -> Cave {
		match cave {
			"start" => Cave::Start,
			"end" => Cave::End,
			small if small == small.to_lowercase() => {
				let mut h = DefaultHasher::new();
				small.hash(&mut h);
				Cave::Small(h.finish())
			},
			big if big == big.to_uppercase() => {
				let mut h = DefaultHasher::new();
				big.hash(&mut h);
				Cave::Big(h.finish())
			},
			_ => unreachable!(),
		}
	}
}

impl Cave {
	const fn to_pow_2(self, index: u32) -> usize {
		match self {
			Cave::Start => 2_usize.pow(1),
			Cave::End => 2_usize.pow(2),
			Cave::Small(_) | Cave::Big(_) => 2_usize.pow(index),
		}
	}
}

fn expand_path<'a>(
	map: &'a BTreeMap<Cave, Vec<Cave>>,
	indexes: &'a BTreeMap<Cave, u32>,
	cave: &'a Cave,
	visited: usize,
	callback: DFSFunc<'a>,
	repeated: bool,
) -> usize {
	map.get(cave).map_or(0, |edges| {
		edges
			.par_iter()
			.map(|to| callback(map, indexes, to, visited | cave.to_pow_2(indexes[cave]), repeated))
			.sum()
	})
}

fn find_paths_to_end<'a>(
	map: &'a BTreeMap<Cave, Vec<Cave>>,
	indexes: &'a BTreeMap<Cave, u32>,
	from: &'a Cave,
	visited: usize,
	repeated: bool,
) -> usize {
	match from {
		Cave::End => 1,
		Cave::Start if visited != 0 => 0,
		cave @ Cave::Small(_) if repeated && visited & cave.to_pow_2(indexes[cave]) > 0 => 0,
		cave @ Cave::Small(_) if !repeated && visited & cave.to_pow_2(indexes[cave]) > 0 => {
			expand_path(map, indexes, cave, visited, find_paths_to_end, true)
		},
		cave => expand_path(map, indexes, cave, visited, find_paths_to_end, repeated),
	}
}

pub fn calculate_paths(input: &str) -> usize {
	let map = input
		.lines()
		.filter_map(|line| line.split_once('-'))
		.fold(BTreeMap::new(), |mut map, (from, to)| {
			let from = Cave::from(from);
			let to = Cave::from(to);
			map.entry(from).or_insert_with(Vec::new).push(to);
			map.entry(to).or_insert_with(Vec::new).push(from);
			map
		});

	let indexes = map.iter().enumerate().map(|(i, (node, _))| (*node, i as u32)).collect();

	find_paths_to_end(&map, &indexes, &Cave::Start, 0, true)
}

pub fn calculate_paths_repeating(input: &str) -> usize {
	let map = input
		.lines()
		.filter_map(|line| line.split_once('-'))
		.fold(BTreeMap::new(), |mut map, (from, to)| {
			let from = Cave::from(from);
			let to = Cave::from(to);
			map.entry(from).or_insert_with(Vec::new).push(to);
			map.entry(to).or_insert_with(Vec::new).push(from);
			map
		});

	let indexes = map.iter().enumerate().map(|(i, (node, _))| (*node, i as u32)).collect();

	find_paths_to_end(&map, &indexes, &Cave::Start, 0, false)
}

pub fn main() {
	let input = read_to_string("input/day12/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_paths(&input));
	println!("PART 2 = {}", calculate_paths_repeating(&input));
	println!("Execution time: {:?}", now.elapsed());
}
