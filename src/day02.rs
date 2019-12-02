
use std::fs;

#[allow(dead_code)]
pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string("src/inputs/day02.txt")
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut memory = input.clone();
    run(12, 2, &mut memory);
    let p1 = memory[0];
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = input.clone();
            run(noun, verb, &mut memory);
            if memory[0] == 19690720 {
                return (p1, 100 * noun + verb);
            }
        }
    }
    unreachable!();
} // 59.51ms

fn run(noun: usize, verb: usize, memory: &mut Vec<usize>) {
    memory[1] = noun;
    memory[2] = verb;
    let mut idx = 0;
    while idx < memory.len() {
        let (a1, a2, pos) = (memory[idx + 1], memory[idx + 2], memory[idx + 3]);
        match memory[idx] {
            1 => memory[pos] = memory[a1] + memory[a2],
            2 => memory[pos] = memory[a1] * memory[a2],
            99 => break,
            _ => panic!("Something went wrong :(")
        }
        idx += 4;
    }
}
