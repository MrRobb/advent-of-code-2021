# Advent of Code 2021

[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/MrRobb/advent-of-code-2019/blob/master/LICENSE)

|        | Problem                                            | Solution                                                                         | Execution time        | Finished |
|--------|----------------------------------------------------|----------------------------------------------------------------------------------|-----------------------|----------|
| Day 1  | [Problem 1](https://adventofcode.com/2021/day/1)   | [day1.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day1.rs)   | 26.920 μs + 26.927 μs | ✓        |
| Day 2  | [Problem 2](https://adventofcode.com/2021/day/2)   | [day2.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day2.rs)   | 25.295 μs + 29.425 μs | ✓        |
| Day 3  | [Problem 3](https://adventofcode.com/2021/day/3)   | [day3.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day3.rs)   | 53.085 μs + 496.60 μs | ✓        |
| Day 4  | [Problem 4](https://adventofcode.com/2021/day/4)   | [day4.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day4.rs)   | 418.32 μs + 1.8037 ms | ✓        |
| Day 5  | [Problem 5](https://adventofcode.com/2021/day/5)   | [day5.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day5.rs)   | 748.14 μs + 848.71 μs | ✓        |
| Day 6  | [Problem 6](https://adventofcode.com/2021/day/6)   | [day6.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day6.rs)   | 2.7375 μs + 2.8628 μs | ✓        |
| Day 7  | [Problem 7](https://adventofcode.com/2021/day/7)   | [day7.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day7.rs)   | 23.143 μs + 15.143 μs | ✓        |
| Day 8  | [Problem 8](https://adventofcode.com/2021/day/8)   | [day8.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day8.rs)   | 16.999 μs + 56.899 μs | ✓        |
| Day 9  | [Problem 9](https://adventofcode.com/2021/day/9)   | [day9.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day9.rs)   | 75.215 μs + 205.23 μs | ✓        |
| Day 10 | [Problem 10](https://adventofcode.com/2021/day/10) | [day10.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day10.rs) | 28.089 μs + 28.422 μs | ✓        |
| Day 11 | [Problem 11](https://adventofcode.com/2021/day/11) | [day11.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day11.rs) | 59.261 μs + 200.58 μs | ✓        |
| Day 12 | [Problem 12](https://adventofcode.com/2021/day/12) | [day12.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day12.rs) | 279.21 μs + 5.0878 ms | ✓        |
| Day 13 | [Problem 13](https://adventofcode.com/2021/day/13) | [day13.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day13.rs) | 106.94 μs + 160.80 μs | ✓        |
| Day 14 | [Problem 14](https://adventofcode.com/2021/day/14) | [day14.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day14.rs) | 68.580 μs + 272.22 μs | ✓        |
| Day 15 | [Problem 15](https://adventofcode.com/2021/day/15) | [day15.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day15.rs) | 1.6114 ms + 22.996 ms | ✓        |
| Day 16 | [Problem 16](https://adventofcode.com/2021/day/16) | [day16.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day16.rs) | 28.969 μs + 29.466 μs | ✓        |
| Day 17 | [Problem 17](https://adventofcode.com/2021/day/17) | [day17.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day17.rs) | 154.66 ns + 4.3031 ms | ✓        |
| Day 18 |                                                    |                                                                                  |                       |          |
| Day 19 |                                                    |                                                                                  |                       |          |
| Day 20 |                                                    |                                                                                  |                       |          |
| Day 21 |                                                    |                                                                                  |                       |          |
| Day 22 |                                                    |                                                                                  |                       |          |
| Day 23 |                                                    |                                                                                  |                       |          |
| Day 24 |                                                    |                                                                                  |                       |          |
| Day 25 |                                                    |                                                                                  |                       |          |

> The benchmarks are measured (non-scientifically) with [cargo-criterion](https://github.com/bheisler/cargo-criterion) on a Macbook Pro 13' M1.

### Benchmark all

```sh
Benchmark 1: ./target/release/advent-of-code-2021
  Time (mean ± σ):      12.6 ms ±   0.4 ms    [User: 45.9 ms, System: 3.0 ms]
  Range (min … max):    12.1 ms …  14.5 ms    200 runs
```

## Install Rust

If you don't have Rust installed ([how dare you](https://media.giphy.com/media/U1aN4HTfJ2SmgB2BBK/giphy.gif)) just run this:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> If you are not using a Unix-like OS, check the instructions [here](https://www.rust-lang.org/tools/install)
## Usage

### Clone

```sh
git clone https://github.com/MrRobb/advent-of-code-2021.git
cd advent-of-code-2021
```

### Build

```sh
cargo build --release
```

### Run

#### Run all

```sh
cargo run --release
```

#### Run a specific day

```sh
cargo run --release --bin day1
```
