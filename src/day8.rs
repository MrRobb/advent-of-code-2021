#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc
)]

use std::{fs::read_to_string};

use itertools::Itertools;

pub fn calculate_1478(input: &str) -> usize {
	input.lines()
        .flat_map(|line| line
            .split_once('|')
            .unwrap()
            .1
            .split(' ')
        )
        .filter(|n| !n.is_empty())
        .filter(|n| {
            n.len() == 2 || n.len() == 4 || n.len() == 3 || n.len() == 7
        })
        .count()
}

fn decode_input(input: &str) -> Vec<String> {
    let input_vec = input
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let mut results = vec![String::new(); 10];
    
    results[1] = (*input_vec.iter().find(|s| s.len() == 2).unwrap()).into();
    results[4] = (*input_vec.iter().find(|s| s.len() == 4).unwrap()).into();
    results[7] = (*input_vec.iter().find(|s| s.len() == 3).unwrap()).into();
    results[8] = (*input_vec.iter().find(|s| s.len() == 7).unwrap()).into();

    let mappings = "abcdefg".chars()
        .permutations(7)
        .find(|s| {
            results[1].chars().contains(&s[2]) &&
            results[1].chars().contains(&s[5]) &&
            
            results[4].chars().contains(&s[1]) &&
            results[4].chars().contains(&s[2]) &&
            results[4].chars().contains(&s[3]) &&
            results[4].chars().contains(&s[5]) &&

            results[7].chars().contains(&s[2]) &&
            results[7].chars().contains(&s[5]) &&
            results[7].chars().contains(&s[0]) &&

            input_vec.iter().filter(|n| n.len() == 5).all(|n| n.chars().contains(&s[0])) &&
            input_vec.iter().filter(|n| n.len() == 5).all(|n| n.chars().contains(&s[3])) &&
            input_vec.iter().filter(|n| n.len() == 5).all(|n| n.chars().contains(&s[6])) &&

            input_vec.iter().filter(|n| n.len() == 6).all(|n| n.chars().contains(&s[0])) &&
            input_vec.iter().filter(|n| n.len() == 6).all(|n| n.chars().contains(&s[1])) &&
            input_vec.iter().filter(|n| n.len() == 6).all(|n| n.chars().contains(&s[5])) &&
            input_vec.iter().filter(|n| n.len() == 6).all(|n| n.chars().contains(&s[6]))
        })
        .unwrap();
        
    results[0] = (*input_vec.iter().filter(|n| n.len() == 6).find(|s| s.chars().all(|c| c != mappings[3])).unwrap()).into();
    results[2] = (*input_vec.iter().filter(|n| n.len() == 5).find(|s| s.chars().all(|c| c != mappings[1] && c != mappings[5])).unwrap()).into();
    results[3] = (*input_vec.iter().filter(|n| n.len() == 5).find(|s| s.chars().all(|c| c != mappings[1] && c != mappings[4])).unwrap()).into();
    results[5] = (*input_vec.iter().filter(|n| n.len() == 5).find(|s| s.chars().all(|c| c != mappings[2] && c != mappings[4])).unwrap()).into();
    results[6] = (*input_vec.iter().filter(|n| n.len() == 6).find(|s| s.chars().all(|c| c != mappings[2])).unwrap()).into();
    results[9] = (*input_vec.iter().filter(|n| n.len() == 6).find(|s| s.chars().all(|c| c != mappings[4])).unwrap()).into();

    results.iter().map(|s| s.chars().sorted().collect()).collect()
}

fn decode_output(output: &str, mappings: &[String]) -> usize {
    output
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| mappings.iter().find_position(|&n| n == &s.chars().sorted().collect::<String>()).unwrap().0)
        .fold(0, |acc, n| acc * 10 + n)
}

pub fn calculate_output(input: &str) -> usize {
	input.lines()
        .map(|line| {
            let (input, output) = line
                .split_once('|')
                .unwrap();
            let mappings = decode_input(input);
            decode_output(output, &mappings)
        })
        .sum::<usize>()
}

pub fn main() {
	let input = read_to_string("input/day8/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_1478(&input));
	println!("PART 2 = {}", calculate_output(&input));
	println!("Execution time: {:?}", now.elapsed());
}
