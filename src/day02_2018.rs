use crate::solution::Solution;

use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve(input: String) -> Solution<i32, String> {
    let (mut twice, mut thrice) = (0, 0);
    input.lines().for_each(|line| {
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
                let p2 = w1
                    .chars()
                    .zip(w2.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect();
                return Solution::new(twice * thrice, p2);
            }
        }
    }
    unreachable!();
} // 114.77ms

#[test]
fn example1() {
    assert_eq!(
        solve(String::from(
            "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"
        ))
        .part1,
        12
    );
}

#[test]
fn example2() {
    assert_eq!(
        solve(String::from(
            "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"
        ))
        .part2,
        "fgij"
    );
}
