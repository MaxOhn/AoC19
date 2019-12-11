use crate::computer::Computer;
use crate::solution::Solution;

use itertools::Itertools;

pub fn solve(input: String) -> Solution<i64, i64> {
    let program: Vec<i64> = input.split(",").map(|n| n.parse().unwrap()).collect();
    let mut p1 = 0;
    for phases in (0..5).permutations(5) {
        let mut amplifiers: Vec<Computer> = phases
            .iter()
            .map(|&phase| {
                let mut computer = Computer::new(program.clone());
                computer.insert(phase);
                computer
            })
            .collect();
        let mut signal = 0;
        for amplifier in &mut amplifiers {
            amplifier.insert(signal).run();
            signal = amplifier.pop();
        }
        p1 = p1.max(signal);
    }
    let mut p2 = 0;
    for phases in (5..10).permutations(5) {
        let mut amplifiers: Vec<Computer> = phases
            .iter()
            .map(|&phase| {
                let mut computer = Computer::new(program.clone());
                computer.insert(phase);
                computer
            })
            .collect();
        let mut idx = 0;
        let mut signal = 0;
        loop {
            amplifiers[idx].insert(signal).run();
            match amplifiers[idx].try_pop() {
                Some(output) => signal = output,
                None => break,
            }
            p2 = p2.max(signal);
            idx = (idx + 1) % 5;
        }
    }
    Solution::new(p1, p2)
} // 203.3ms

#[test]
fn example1() {
    let input = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(solve(input).part1, 43210);
}

#[test]
fn example2() {
    let input = String::from(
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
    );
    assert_eq!(solve(input).part2, 139629729);
}
