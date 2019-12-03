
mod day01;
mod day01_2018;
mod day02;
mod day02_2018;
mod day03;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!(
        "Solution: {:?} [Elapsed time: {:?}]",
        day03::solve(),
        Instant::now().checked_duration_since(start).unwrap()
    );
}
