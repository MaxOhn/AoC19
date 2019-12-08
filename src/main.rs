
use std::fs;
use std::env;
use std::time::Instant;

use aoc19;

fn main() {
    let days = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let day: i32 = if let Some(arg) = env::args().nth(1) {
        arg.parse().unwrap_or(*days.last().unwrap())
    } else {
        *days.last().unwrap()
    };
    let input = fs::read_to_string(format!("inputs/day{:0>2}.txt", day))
        .expect("Error: Invalid day");
    let start = Instant::now();
    let solution = match day {
        1 => aoc19::day01::solve(input).to_string(),
        2 => aoc19::day02::solve(input).to_string(),
        3 => aoc19::day03::solve(input).to_string(),
        4 => aoc19::day04::solve(input).to_string(),
        5 => aoc19::day05::solve(input).to_string(),
        6 => aoc19::day06::solve(input).to_string(),
        7 => aoc19::day07::solve(input).to_string(),
        8 => aoc19::day08::solve(input).to_string(),
        d if 0 < d && d < 26 => panic!("Error: Day {} not yet implemented", d),
        _ => panic!("Error: Invalid day"),
    };
    println!(
        "[Day {}] Elapsed time: {:?}\n{}", day,
        Instant::now().checked_duration_since(start).unwrap(),
        solution,
    );
}
