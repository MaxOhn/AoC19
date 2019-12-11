
use crate::solution::Solution;
use crate::computer::{
    Computer,
    Channel,
};

use std::thread;
use itertools::Itertools;
use std::sync::{
    Arc,
    Barrier,
};

use crossbeam::channel;

pub fn solve(input: String) -> Solution<i64, i64> {
    let program: Vec<i64> = input.split(",")
        .map(|n| n.parse().unwrap())
        .collect();
        
    let (tx_out, rx_out) = channel::bounded(120);
    let mut handles = Vec::new();
    let mut senders = Vec::new();
    let barrier = Arc::new(Barrier::new(5));
    for i in 0..5 {
        let (tx_in, rx_in) = channel::bounded(120);
        senders.push(tx_in);
        let program = program.clone();
        let barrier = barrier.clone();
        let tx_out = tx_out.clone();
        let handle = thread::spawn(move || {
            loop {
                let (part, phase, input, output) = match rx_in.recv() {
                    Ok(data) => data,
                    Err(_) => break,
                };
                let mut computer = Computer::new(program.clone());
                computer.set_input_channel(input)
                    .set_output_channel(output)
                    .insert(phase);
                if i == 0 {
                    computer.insert(0);
                }
                barrier.wait();
                computer.run();
                if i == 4 {
                    let result = computer.pop();
                    tx_out.send((part, result)).unwrap();
                }
            }
        });
        handles.push(handle);
    }

    // Part 1
    for phases in (0..5).permutations(5) {
        let channels: Vec<_> = (0..5)
            .map(|_| Channel::default())
            .collect();
        let mut outputs = (0..5).map(|i| channels[i].clone());
        let mut inputs = (0..5).map(|i| channels[(i + 4) % 5].clone());
        for i in 0..5 {
            let output = outputs.next().unwrap();
            let input = inputs.next().unwrap();
            senders[i].send((1, phases[i], input, output)).unwrap();
        }
    }
    // Part 2
    for phases in (5..10).permutations(5) {
        let channels: Vec<_> = (0..5)
            .map(|_| Channel::default())
            .collect();
        let mut outputs = (0..5).map(|i| channels[i].clone());
        let mut inputs = (0..5).map(|i| channels[(i + 4) % 5].clone());
        for i in 0..5 {
            let output = outputs.next().unwrap();
            let input = inputs.next().unwrap();
            senders[i].send((2, phases[i], input, output)).unwrap();
        }
    }

    drop(senders);
    drop(tx_out);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut iter = rx_out.iter();
    while let Some((part, output)) = iter.next() {
        match part {
            1 => p1 = p1.max(output),
            2 => p2 = p2.max(output),
            _ => unreachable!(),
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Solution::new(p1, p2)
} // 186.36ms

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
