use crate::Solution;

pub fn solve(input: String) -> Solution<i32, i32> {
    let signal = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();
    let p1 = solve_part1(signal.clone());
    let p2 = solve_part2(signal);
    Solution::new(p1, p2)
} // 7.42s

fn solve_part1(mut signal: Vec<i32>) -> i32 {
    let mut next = vec![0; signal.len()];
    for _ in 0..100 {
        for i in 1..=signal.len() {
            let mut sum = 0;
            for j in 0..signal.len() {
                let pattern_elem = match (j + 1) % (4 * i) {
                    x if x < i => continue,
                    x if x < 2 * i => 1,
                    x if x < 3 * i => continue,
                    _ => -1,
                };
                sum += signal[j] * pattern_elem;
            }
            next[i - 1] = sum.abs() % 10;
        }
        for i in 0..signal.len() {
            signal[i] = next[i];
        }
    }
    signal
        .into_iter()
        .take(8)
        .fold(0, |sum, next| 10 * sum + next)
}

fn solve_part2(mut signal: Vec<i32>) -> i32 {
    let offset = signal[..7].iter().fold(0, |sum, &next| 10 * sum + next) as usize;
    assert!(offset > signal.len() / 2);
    let len = signal.len() * 10_000 - offset;
    signal = signal.into_iter().rev().cycle().take(len).collect();
    for _ in 0..100 {
        signal = signal
            .iter()
            .scan(0, |sum, curr| {
                *sum += curr;
                Some(*sum % 10)
            })
            .collect();
    }
    signal
        .into_iter()
        .rev()
        .take(8)
        .fold(0, |sum, next| 10 * sum + next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test16() {
        let inputs: Vec<(&str, i32, Box<dyn Fn(Vec<i32>) -> i32>)> = vec![
            (
                "80871224585914546619083218645595",
                24176176,
                Box::new(solve_part1),
            ),
            (
                "19617804207202209144916044189917",
                73745418,
                Box::new(solve_part1),
            ),
            (
                "69317163492948606335995924319873",
                52432133,
                Box::new(solve_part1),
            ),
            (
                "03036732577212944063491565474664",
                84462026,
                Box::new(solve_part2),
            ),
            (
                "02935109699940807407585447034323",
                78725270,
                Box::new(solve_part2),
            ),
            (
                "03081770884921959731165446850517",
                53553731,
                Box::new(solve_part2),
            ),
        ];
        for input in &inputs {
            let signal = input
                .0
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>();
            assert_eq!(input.2(signal), input.1);
        }
        crate::util::tests::test_full_problem(16, solve, 36627552, 79723033);
    }
}
