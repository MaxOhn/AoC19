
use std::fs;
use std::collections::HashMap;
 
#[allow(dead_code)]
pub fn solve() -> (i32, String) {
    let (mut twice, mut thrice) = (0, 0);
    let input = fs::read_to_string("src/inputs/day02_2018.txt").unwrap();
    input.lines()
        .for_each(|line| {
            let mut characters = HashMap::with_capacity(26);
            line.chars().for_each(|c| {
                *characters.entry(c).or_insert(0) += 1;
            });
            if characters.values().any(|v| *v == 2) {
                twice += 1;
            }
            if characters.values().any(|v| *v == 3) {
                thrice += 1;
            }
        });
    for (i, w1) in input.lines().enumerate() {
        for w2 in input.lines().skip(i + 1) {
            if w1.chars().zip(w2.chars()).filter(|(a, b)| a != b).count() == 1 {
                return (twice * thrice, 
                    w1.chars()
                        .zip(w2.chars())
                        .filter(|(a, b)| a == b)
                        .map(|(a, _)| a)
                        .collect());
            }
        }
    }
    unreachable!();
} // 114.77ms
