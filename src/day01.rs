
use std::fs;

#[allow(dead_code)]
pub fn solve() -> (i32, i32) {
    let (mut p1, mut p2) = (0, 0);
    fs::read_to_string("src/inputs/day01.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .for_each(|num| {
            let mut n = num / 3 - 2;
            p1 += n;
            while n > 0 {
                p2 += n;
                n = n / 3 - 2;
            }
        });
    (p1, p2)
} // 4.44ms
