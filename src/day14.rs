#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::collections::BTreeMap;
use std::fs::read_to_string;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

pub fn calculate_frequencies(input: &str, steps: usize) -> usize {
	let (template, insertions) = input.split_once("\n\n").unwrap();

	let insertions = insertions
		.lines()
		.filter_map(|line| line.split_once(" -> "))
		.map(|(pair, insertion)| {
			(
				(pair.chars().next().unwrap(), pair.chars().nth(1).unwrap()),
				insertion.chars().next().unwrap(),
			)
		})
		.collect::<BTreeMap<_, _>>();

	let frequencies = template.chars().tuple_windows().map(|(a, b)| (a, b)).counts();

	let x = (0..steps).fold(frequencies, |frequencies, _| {
		let mut new_frequencies = frequencies.clone();
		for pair @ ((a, b), f) in frequencies {
			if let Some(insertion) = insertions.get(&pair.0) {
				*new_frequencies.entry((a, *insertion)).or_insert(0) += f;
				*new_frequencies.entry((*insertion, b)).or_insert(0) += f;
				*new_frequencies.entry((a, b)).or_insert(0) -= f;
			}
		}
		new_frequencies
	});

	let mut letter_frequencies = BTreeMap::new();

	for ((a, b), f) in x {
		letter_frequencies.entry(a).or_insert((0, 0)).0 += f;
		letter_frequencies.entry(b).or_insert((0, 0)).1 += f;
	}

	if let MinMax(min, max) = letter_frequencies.into_iter().map(|(_, (l, r))| l.max(r)).minmax() {
		max - min
	}
	else {
		unreachable!()
	}
}

pub fn main() {
	let input = read_to_string("input/day14/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_frequencies(&input, 10));
	println!("PART 2 = {}", calculate_frequencies(&input, 40));
	println!("Execution time: {:?}", now.elapsed());
}
