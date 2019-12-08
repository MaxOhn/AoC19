
#[macro_use] extern crate cached;

mod day01;
mod day01_2018;
mod day02;
mod day02_2018;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use std::fs;
use std::env;
use std::time::Instant;

fn main() {
    let days = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let day: i32 = if let Some(arg) = env::args().nth(1) {
        arg.parse().unwrap_or(*days.last().unwrap())
    } else {
        *days.last().unwrap()
    };
    let input = fs::read_to_string(format!("src/inputs/day{:0>2}.txt", day))
        .expect("Error: Invalid day");
    let start = Instant::now();
    println!(
        "[Day {}]\n{}\n[Elapsed time: {:?}]",
        day,
        match day {
            1 => day01::solve(input).to_string(),
            2 => day02::solve(input).to_string(),
            3 => day03::solve(input).to_string(),
            4 => day04::solve(input).to_string(),
            5 => day05::solve(input).to_string(),
            6 => day06::solve(input).to_string(),
            7 => day07::solve(input).to_string(),
            8 => day08::solve(input).to_string(),
            _ => panic!("Error: Invalid day"),
        },
        Instant::now().checked_duration_since(start).unwrap()
    );
}
