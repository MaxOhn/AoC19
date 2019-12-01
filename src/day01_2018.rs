
use std::fs;
use std::collections::HashSet;
use std::cell::Cell;

#[allow(dead_code)]
pub fn solve() -> (i32, i32) {
    let input: Vec<i32> = fs::read_to_string("src/inputs/day01_2018.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let p1 = input.iter().sum::<i32>();
    let p2 = Cell::new(0);
    let mut seen = HashSet::new();
    input.iter()
        .cycle()
        .take_while(|_| seen.insert(p2.get()))
        .for_each(|n| p2.set(p2.get() + n));
    (p1, p2.get())
} // 236.41ms
