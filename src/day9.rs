#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::fs::read_to_string;

use itertools::Itertools;

pub fn calculate_low_points(input: &str) -> u32 {
	let heightmap = input.lines()
        .map(|s| s.chars().filter_map(|d| d.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let mut risk = 0;
    let empty = Vec::new();
    for row in 0..heightmap.len() {
        for col in 0..heightmap[row].len() {
            let point = &heightmap[row][col];
            let up = if row >= 1 { heightmap.get(row - 1).unwrap_or(&empty).get(col).unwrap_or(&10) } else { &10 };
            let down = heightmap.get(row + 1).unwrap_or(&empty).get(col).unwrap_or(&10);
            let left = if col >= 1 { heightmap.get(row).unwrap_or(&empty).get(col - 1).unwrap_or(&10) } else { &10 };
            let right = heightmap.get(row).unwrap_or(&empty).get(col + 1).unwrap_or(&10);
            if point < up && point < down && point < left && point < right {
                risk += point + 1;
            }
        }
    }
    risk
}

pub fn visit_node(point: &(usize, usize), heightmap: &[Vec<u32>], visited: &mut Vec<Vec<bool>>) -> u32 {
    if heightmap[point.0][point.1] == 9 || visited[point.0][point.1] { 0 } else {
        visited[point.0][point.1] = true;

        1 + 
        if point.0 >= 1 { visit_node(&(point.0 - 1, point.1), heightmap, visited) } else { 0 } +
        if point.0 + 1 < heightmap.len() { visit_node(&(point.0 + 1, point.1), heightmap, visited) } else { 0 } +
        if point.1 >= 1 { visit_node(&(point.0, point.1 - 1), heightmap, visited) } else { 0 } +
        if point.1 + 1 < heightmap[0].len() { visit_node(&(point.0, point.1 + 1), heightmap, visited) } else { 0 }
    }
}

pub fn calculate_basins(input: &str) -> u32 {
	let heightmap = input.lines()
        .map(|s| s.chars().filter_map(|d| d.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let mut lowpoints = Vec::new();
    let empty = Vec::new();
    for row in 0..heightmap.len() {
        for col in 0..heightmap[row].len() {
            let point = &heightmap[row][col];
            let up = if row >= 1 { heightmap.get(row - 1).unwrap_or(&empty).get(col).unwrap_or(&10) } else { &10 };
            let down = heightmap.get(row + 1).unwrap_or(&empty).get(col).unwrap_or(&10);
            let left = if col >= 1 { heightmap.get(row).unwrap_or(&empty).get(col - 1).unwrap_or(&10) } else { &10 };
            let right = heightmap.get(row).unwrap_or(&empty).get(col + 1).unwrap_or(&10);
            if point < up && point < down && point < left && point < right {
                lowpoints.push((row, col));
            }
        }
    }

    let mut visited = vec![vec![false; heightmap[0].len()]; heightmap.len()];
    lowpoints.iter()
        .map(|point| visit_node(point, &heightmap, &mut visited))
        .sorted_unstable()
        .rev()
        .take(3)
        .product()
}

pub fn main() {
	let input = read_to_string("input/day9/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_low_points(&input));
	println!("PART 2 = {}", calculate_basins(&input));
	println!("Execution time: {:?}", now.elapsed());
}
