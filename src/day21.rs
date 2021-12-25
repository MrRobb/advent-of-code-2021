#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::fs::read_to_string;

use cached::proc_macro::cached;

fn play(mut player1: usize, mut player2: usize) -> (usize, usize) {
	let mut dice_rolls = 0;
	let mut score1 = 0;
	let mut score2 = 0;

	let mut dice = std::iter::successors(Some((1, 2, 3)), |(_, _, c)| {
		Some(((c + 1) % 100, (c + 2) % 100, (c + 3) % 100))
	});

	let mut turn_of_1 = true;
	while score1 < 1000 && score2 < 1000 {
		let rolls = dice.next().unwrap();
		if turn_of_1 {
			player1 = 1 + ((player1 - 1) + rolls.0 + rolls.1 + rolls.2) % 10;
			score1 += player1;
		}
		else {
			player2 = 1 + ((player2 - 1) + rolls.0 + rolls.1 + rolls.2) % 10;
			score2 += player2;
		}
		turn_of_1 = !turn_of_1;
		dice_rolls += 3;
	}

	println!("{:?}, {}", (score1, score2), dice_rolls);

	(score1.min(score2), dice_rolls)
}

#[cached]
fn play_quantum(score1: usize, score2: usize, player1: usize, player2: usize, turn_of_1: bool) -> (usize, usize) {
	if score1 >= 21 {
		return (1, 0);
	}
	if score2 >= 21 {
		return (0, 1);
	}

	[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
		.into_iter()
		.map(|(roll_sum, times)| {
			let (score1, score2) = if turn_of_1 {
				let player1 = 1 + ((player1 - 1) + roll_sum) % 10;
				play_quantum(score1 + player1, score2, player1, player2, false)
			}
			else {
				let player2 = 1 + ((player2 - 1) + roll_sum) % 10;
				play_quantum(score1, score2 + player2, player1, player2, true)
			};
			(times * score1, times * score2)
		})
		.fold((0, 0), |scores, score| (scores.0 + score.0, scores.1 + score.1))
}

pub fn calculate_play(input: &str) -> usize {
	let players = input
		.lines()
		.flat_map(|line| line.split_once("starting position: ").unwrap().1.parse::<usize>())
		.collect::<Vec<_>>();

	let (score, dice_rolls) = play(players[0], players[1]);
	score * dice_rolls
}

pub fn calculate_quantum(input: &str) -> usize {
	let players = input
		.lines()
		.flat_map(|line| line.split_once("starting position: ").unwrap().1.parse::<usize>())
		.collect::<Vec<_>>();

	let (score1, score2) = play_quantum(0, 0, players[0], players[1], true);
	score1.max(score2)
}

pub fn main() {
	let input = read_to_string("input/day21/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_play(&input));
	println!("PART 2 = {:?}", calculate_quantum(&input));
	println!("Execution time: {:?}", now.elapsed());
}
