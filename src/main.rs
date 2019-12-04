
mod day01;
mod day01_2018;
mod day02;
mod day02_2018;
mod day03;
mod day04;

use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!(
        "Solution: {:?} [Elapsed time: {:?}]",
        day04::solve(fs::read_to_string("src/inputs/day04.txt").unwrap()),
        Instant::now().checked_duration_since(start).unwrap()
    );
}
