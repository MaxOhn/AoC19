use std::env;
use std::fs;
use std::time::Instant;

use aoc19::*;

fn main() {
    let days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    let day: i32 = if let Some(arg) = env::args().nth(1) {
        arg.parse().unwrap_or(*days.last().unwrap())
    } else {
        *days.last().unwrap()
    };
    let input =
        fs::read_to_string(format!("inputs/day{:02}.txt", day)).expect("Error: Invalid day");
    let start = Instant::now();
    let solution = match day {
        1 => day01::solve(input).to_string(),
        2 => day02::solve(input).to_string(),
        3 => day03::solve(input).to_string(),
        4 => day04::solve(input).to_string(),
        5 => day05::solve(input).to_string(),
        6 => day06::solve(input).to_string(),
        7 => day07::solve(input).to_string(),
        8 => day08::solve(input).to_string(),
        9 => day09::solve(input).to_string(),
        10 => day10::solve(input).to_string(),
        11 => day11::solve(input).to_string(),
        12 => day12::solve(input).to_string(),
        13 => day13::solve(input).to_string(),
        d if 0 < d && d < 26 => panic!("Error: Day {} not yet implemented", d),
        _ => panic!("Error: Invalid day"),
    };
    println!(
        "[Day {}] Elapsed time: {:?}\n{}",
        day,
        Instant::now().checked_duration_since(start).unwrap(),
        solution,
    );
}
