use std::fs::read_to_string;

pub fn calculate_lanternfishes(input: &str, days: usize) -> usize {
	let mut lanternfishes = input
		.split(',')
		.map(|s| s.parse::<usize>().unwrap())
		.fold([0; 9], |mut acc, n| {
			acc[n] += 1;
			acc
		});

	for _ in 0..days {
		let mut aux = [0; 9];
		aux[0] = lanternfishes[1];
		aux[1] = lanternfishes[2];
		aux[2] = lanternfishes[3];
		aux[3] = lanternfishes[4];
		aux[4] = lanternfishes[5];
		aux[5] = lanternfishes[6];
		aux[6] = lanternfishes[7] + lanternfishes[0];
		aux[7] = lanternfishes[8];
		aux[8] = lanternfishes[0];
		lanternfishes = aux;
	}

	lanternfishes.iter().sum()
}

pub fn main() {
	let input = read_to_string("input/day6/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_lanternfishes(&input, 80));
	println!("PART 2 = {}", calculate_lanternfishes(&input, 256));
	println!("Execution time: {:?}", now.elapsed())
}
