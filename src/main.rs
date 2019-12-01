
mod day01;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!(
        "Solution: {:?} [Elapsed time: {:?}]",
        day01::solve(),
        Instant::now().checked_duration_since(start).unwrap()
    );
}

