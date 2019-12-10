
use crate::solution::Solution;
use crate::computer::Computer;

use std::thread;
use itertools::Itertools;
use std::sync::{
    Arc,
    Barrier,
};

pub fn solve(input: String) -> Solution<i64, i64> {
    let program: Vec<i64> = input.split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    let mut handles = Vec::new();
    for i in 0..5 {
        let handle = thread::spawn(move || {
            // TODO
        });
        handles.push(handle);
    }
    let p1 = 0;
    /*
    let mut p1 = 0;
    for permutation in (0..5).permutations(5) {
        let mut amplifiers: Vec<Computer> = permutation
            .iter()
            .map(|&phase| {
                let mut computer = Computer::new(program.clone());
                computer.insert(phase);
                computer
            })
            .collect();
        let mut signal = 0;
        for amplifier in &mut amplifiers {
            signal = amplifier.insert(signal).pop();
        }
        p1 = i64::max(p1, signal);
    }
    */
    let mut p2 = 0;
    for permutation in (5..10).permutations(5) {
        let mut amplifiers: Vec<Computer> = permutation
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
            /*
            amplifiers[idx].insert(signal);
            match amplifiers[idx].pop() {
                Some(output) => {
                    signal = output;
                    idx = (idx + 1) % 5;
                    p2 = i64::max(p2, signal);
                },
                None => break,
            }
            */
            break;
        }
    }
    Solution::new(p1, p2)
} // 10.73ms

#[test]
fn example1() {
    let input = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(solve(input).part1, 43210);
}

#[test]
fn example2() {
    let input = String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    assert_eq!(solve(input).part2, 139629729);
}
