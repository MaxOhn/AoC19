
use aoc19::Solution;

use permutohedron::Heap;

pub fn solve(input: String) -> Solution<i32, i32> {
    let program: Vec<i32> = input.split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    let mut p1 = 0;
    let mut phases = vec![0, 1, 2, 3, 4];
    let heap = Heap::new(&mut phases);
    for permutation in heap {
        let mut amplifiers: Vec<Amplifier> = permutation.iter()
            .map(|&phase| Amplifier::new(program.clone(), phase))
            .collect();
        let mut signal = 0;
        for amplifier in &mut amplifiers {
            signal = amplifier.run(signal).unwrap();
        }
        p1 = i32::max(p1, signal);
    }
    let mut p2 = 0;
    let mut phases = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut phases);
    for permutation in heap {
        let mut amplifiers: Vec<Amplifier> = permutation.iter()
            .map(|&phase| Amplifier::new(program.clone(), phase))
            .collect();
        let mut signal = 0;
        let mut idx = 0;
        while let Some(output) = amplifiers[idx].run(signal) {
            signal = output;
            idx = (idx + 1) % 5;
            p2 = i32::max(p2, signal);
        }
    }
    Solution::new(p1, p2)
} // 3.75ms

struct Amplifier {
    memory: Vec<i32>,
    pc: usize,
}

impl Amplifier {
    fn new(memory: Vec<i32>, phase: i32) -> Self {
        let mut amplifier = Amplifier { memory, pc: 0 };
        amplifier.run(phase);
        amplifier
    }

    fn run(&mut self, input: i32) -> Option<i32> {
        if self.memory[self.pc] == 99 { return None; }
        let input_pos = self.memory[self.pc + 1] as usize;
        self.memory[input_pos] = input;
        self.pc += 2;
        let mut output = None;
        while let Some(mut op) = Operation::new(&self.memory, self.pc) {
            match op.opcode {
                1 => self.memory[op.w] = op.v1 + op.v2,
                2 => self.memory[op.w] = op.v1 * op.v2,
                3 => break,
                4 => output = Some(op.v1),
                5 => op.pc = if op.v1 != 0 { op.v2 as usize } else { op.pc + 3 },
                6 => op.pc = if op.v1 == 0 { op.v2 as usize } else { op.pc + 3 },
                7 => self.memory[op.w] = if op.v1 < op.v2 { 1 } else { 0 },
                8 => self.memory[op.w] = if op.v1 == op.v2 { 1 } else { 0 },
                _ => unreachable!()
            }
            self.pc = op.pc;
        }
        output
    }
}

struct Operation {
    opcode: i32,
    v1: i32,
    v2: i32,
    w: usize,
    pc: usize
}

impl Operation {
    fn new(mem: &[i32], pc: usize) -> Option<Self> {
        let opcode = mem[pc] % 100;
        if opcode == 99 { return None; }
        if opcode == 3 {
            return Some(Operation { opcode, v1: 0, v2: 0, w: mem[pc + 1] as usize, pc: pc + 2 });
        }
        let v1 = if (mem[pc] / 100) % 10 == 0 { mem[mem[pc + 1] as usize] } else { mem[pc + 1] };
        if opcode == 4 {
            return Some(Operation { opcode, v1, v2: 0, w: 0, pc: pc + 2 });
        }
        let v2 = if (mem[pc] / 1000) % 10 == 0 { mem[mem[pc + 2] as usize] } else { mem[pc + 2] };
        match opcode {
            1 | 2 | 7 | 8 => Some(Operation { opcode, v1, v2, w: mem[pc + 3] as usize, pc: pc + 4 }),
            5 | 6 => Some(Operation { opcode, v1, v2, w: 0, pc }),
            _ => panic!("Error: opcode = {}", opcode)
        }
    }
}

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
