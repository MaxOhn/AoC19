
mod day01;
mod day01_2018;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!(
        "Solution: {:?} [Elapsed time: {:?}]",
        day01_2018::solve(),
        Instant::now().checked_duration_since(start).unwrap()
    );
}

