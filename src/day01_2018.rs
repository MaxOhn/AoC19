
use std::fs;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn solve() -> (i32, i32) {
    let input: Vec<i32> = fs::read_to_string("src/inputs/day01_2018.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let p1: i32 = input.iter().sum();
    let mut p2 = 0;
    let mut seen: HashSet<i32> = HashSet::with_capacity(input.len());
    seen.insert(0);
    for num in input.iter().cycle() {
        p2 += num;
        if seen.contains(&p2) {
            return (p1, p2);
        }
        seen.insert(p2);
    };
    (0, 0)
} // 756.601ms