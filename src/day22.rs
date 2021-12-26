#![allow(clippy::must_use_candidate, clippy::missing_panics_doc, clippy::cast_precision_loss)]

use std::fs::read_to_string;
use std::ops::RangeInclusive;

use euclid::{Box3D, Point3D, UnknownUnit};
use itertools::Itertools;

type Coords = (RangeInclusive<i64>, RangeInclusive<i64>, RangeInclusive<i64>);

fn how_many_on_coords(coords: &[(bool, Coords)], x: RangeInclusive<i64>, y: RangeInclusive<i64>, z: RangeInclusive<i64>) -> usize {
	x.cartesian_product(y)
		.cartesian_product(z)
		.filter(|((a, b), c)| {
			let mut status = false;
			for (status_i, (xi, yi, zi)) in coords {
				if xi.contains(a) && yi.contains(b) && zi.contains(c) {
					status = *status_i;
				}
			}
			status
		})
		.count()
}

fn adjust_cuboids(positives: &[Box3D<i64, UnknownUnit>], negatives: &[Box3D<i64, UnknownUnit>], cube: Box3D<i64, UnknownUnit>) -> (Vec<Box3D<i64, UnknownUnit>>, Vec<Box3D<i64, UnknownUnit>>) {
	
	let pts = negatives
		.iter()
		.filter_map(|c| c.intersection(&cube))
		.collect();

	let ngs = positives
		.iter()
		.filter_map(|c| c.intersection(&cube))
		.collect();

	(pts, ngs)
}

fn how_many_on(coords: &[(bool, Coords)]) -> i64 {
	let mut positives = Vec::new();
	let mut negatives = Vec::new();

	for (status, (x, y , z)) in coords {
		let cube = Box3D::<_, UnknownUnit>::new(
			Point3D::new(*x.start(), *y.start(), *z.start()), 
			Point3D::new(*x.end() + 1, *y.end() + 1, *z.end() + 1), 
		);

		let (mut pts, mut ngs) = adjust_cuboids(&positives, &negatives, cube);

		if *status {
			positives.push(cube);
		}

		positives.append(&mut pts);
		negatives.append(&mut ngs);
	}

	let p = positives.iter().map(euclid::Box3D::volume).sum::<i64>();
	let n = negatives.iter().map(euclid::Box3D::volume).sum::<i64>();
	p - n
}

pub fn calculate_how_many_on_coords(input: &str) -> usize {
	let coords = input
		.lines()
		.filter_map(|line| line.split_once(' '))
		.map(|(status, coords)| {
			(
				status == "on",
				coords
					.split(',')
					.map(|coord_axis| {
						let (_, coords) = coord_axis.split_once("=").unwrap();
						let (from_str, to_str) = coords.split_once("..").unwrap();
						let (from, to) = (from_str.parse().unwrap(), to_str.parse().unwrap());
						from..=to
					})
					.collect_tuple::<Coords>()
					.unwrap(),
			)
		})
		.collect::<Vec<_>>();
		how_many_on_coords(&coords, -50..=50, -50..=50, -50..=50)
}

pub fn calculate_how_many_on(input: &str) -> i64 {
	let coords = input
		.lines()
		.filter_map(|line| line.split_once(' '))
		.map(|(status, coords)| {
			(
				status == "on",
				coords
					.split(',')
					.map(|coord_axis| {
						let (_, coords) = coord_axis.split_once("=").unwrap();
						let (from_str, to_str) = coords.split_once("..").unwrap();
						let (from, to) = (from_str.parse().unwrap(), to_str.parse().unwrap());
						from..=to
					})
					.collect_tuple::<Coords>()
					.unwrap(),
			)
		})
		.collect::<Vec<_>>();
		how_many_on(&coords)
}

pub fn main() {
	let input = read_to_string("input/day22/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_how_many_on_coords(&input));
	println!("PART 2 = {}", calculate_how_many_on(&input));
	println!("Execution time: {:?}", now.elapsed());
}
