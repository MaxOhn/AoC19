
mod day01;
mod day01_2018;
mod day02;
mod day02_2018;
mod day03;
mod day04;
mod day05;
mod day06;

use std::fs;
use std::env;
use std::time::Instant;

fn main() {
    let days: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    let day: i32 = if let Some(arg) = env::args().nth(1) {
        arg.parse().unwrap_or(*days.last().unwrap())
    } else {
        *days.last().unwrap()
    };
    let input = fs::read_to_string(format!("src/inputs/day{:0>2}.txt", day)).unwrap();
    let start = Instant::now();
    println!(
        "Solution: {} [Elapsed time: {:?}]",
        match day {
            1 => format!("{:?}", day01::solve(input)),
            2 => format!("{:?}", day02::solve(input)),
            3 => format!("{:?}", day03::solve(input)),
            4 => format!("{:?}", day04::solve(input)),
            5 => format!("{:?}", day05::solve(input)),
            6 => format!("{:?}", day06::solve(input)),
            _ => panic!("Invalid day"),
        },
        Instant::now().checked_duration_since(start).unwrap()
    );
}
