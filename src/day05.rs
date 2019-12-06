
pub fn solve(input: String) -> (i32, i32) {
    let intcodes: Vec<i32> = input.split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    let p1 = match run(&mut intcodes.clone(), 1) {
        Some(result) => result,
        None => panic!("Error: Got None for part 1")
    };
    let p2 = match run(&mut intcodes.clone(), 5) {
        Some(result) => result,
        None => panic!("Error: Got None for part 2")
    };
    (p1, p2)
} // 0.28ms

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

fn run(memory: &mut [i32], input: i32) -> Option<i32> {
    let mut result = None;
    let mut pc = 0;
    while let Some(mut op) = Operation::new(&memory, pc) {
        match op.opcode {
            1 => memory[op.w] = op.v1 + op.v2,
            2 => memory[op.w] = op.v1 * op.v2,
            3 => memory[op.w] = input,
            4 => result = Some(op.v1),
            5 => op.pc = if op.v1 != 0 { op.v2 as usize } else { op.pc + 3 },
            6 => op.pc = if op.v1 == 0 { op.v2 as usize } else { op.pc + 3 },
            7 => memory[op.w] = if op.v1 < op.v2 { 1 } else { 0 },
            8 => memory[op.w] = if op.v1 == op.v2 { 1 } else { 0 },
            _ => unreachable!()
        }
        pc = op.pc;
    }
    result
}

#[test]
fn example1() {
    let intcodes = [3,9,7,9,10,9,4,9,99,-1,8];
    assert_eq!(run(&mut intcodes.clone(), 7).unwrap(), 1);
    assert_eq!(run(&mut intcodes.clone(), 9).unwrap(), 0);
}

#[test]
fn example2() {
    let intcodes = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    assert_eq!(run(&mut intcodes.clone(), 7).unwrap(), 999);
    assert_eq!(run(&mut intcodes.clone(), 8).unwrap(), 1000);
    assert_eq!(run(&mut intcodes.clone(), 9).unwrap(), 1001);
}