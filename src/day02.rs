
use std::fs;

#[allow(dead_code)]
pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/inputs/day02.txt")
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut memory = input.clone();
    let p1 = run(12, 2, &mut memory);
    for noun in 0..100 {
        for verb in 0..100 {
            memory = input.clone();
            if run(noun, verb, &mut memory) == 19690720 {
                return (p1, 100 * noun + verb);
            }
        }
    }
    unreachable!();
} // 8.52ms

fn run(noun: usize, verb: usize, memory: &mut [usize]) -> usize {
    memory[1] = noun;
    memory[2] = verb;
    let mut i = 0;
    loop {
        let (a1, a2, s) = (memory[i + 1], memory[i + 2], memory[i + 3]);
        match memory[i] {
            99 => break memory[0],
            1 => memory[s] = memory[a1] + memory[a2],
            2 => memory[s] = memory[a1] * memory[a2],
            _ => panic!("Something went wrong :(")
        }
        i += 4;
    }
}
