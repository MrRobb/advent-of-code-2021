use std::fs::read_to_string;

const DIGITS: usize = 12;

pub fn calculate_power_consumption(input: &str) -> usize {
	let mut counts: Vec<i64> = vec![0; DIGITS];

	for line in input.lines() {
		for (i, c) in line.chars().enumerate() {
			match c {
				'0' => counts[i] -= 1,
				'1' => counts[i] += 1,
				_ => unreachable!(),
			}
		}
	}

	let gamma_bits = counts
		.iter()
		.map(|&count| if count > 0 { '1' } else { '0' })
		.collect::<String>();
	let epsilon_bits = counts
		.iter()
		.map(|&count| if count > 0 { '0' } else { '1' })
		.collect::<String>();

	let gamma_rate = usize::from_str_radix(&gamma_bits, 2).unwrap();
	let epsilon_rate = usize::from_str_radix(&epsilon_bits, 2).unwrap();

	gamma_rate * epsilon_rate
}

fn get_prefix(input: &str, f: fn(usize, usize) -> bool) -> usize {
	let mut prefix = String::new();

	for digit in 0..DIGITS {
		let filtered: Vec<_> = input.lines().filter(|line| line.starts_with(&prefix)).collect();

		if filtered.len() == 1 {
			break;
		}

		let count1 = filtered
			.iter()
			.map(|&line| line.chars().nth(digit).unwrap())
			.filter(|&c| c == '1')
			.count();

		prefix.push(if f(count1, filtered.len() - count1) { '1' } else { '0' });
	}

	let filtered: Vec<_> = input.lines().filter(|line| line.starts_with(&prefix)).collect();

	usize::from_str_radix(&filtered[0], 2).unwrap()
}

pub fn filter_power_consumption(input: &str) -> usize {
	let oxygen = get_prefix(input, |count1, count0| count1 >= count0);
	let co2 = get_prefix(input, |count1, count0| count1 < count0);
	oxygen * co2
}

pub fn main() {
	let input = read_to_string("input/day3/input.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_power_consumption(&input));
	println!("PART 2 = {}", filter_power_consumption(&input));
	println!("Execution time: {:?}", now.elapsed())
}
