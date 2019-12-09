
pub(crate) struct Computer {
    memory: Vec<i64>,
    pc: usize,
    pub rb: i32,
    output: Vec<i64>
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Self {
        Computer { memory, pc: 0, rb: 0, output: Vec::new() }
    }
    
    pub fn run_on_input(&mut self, input: i64) {
        if self.memory[self.pc] == 99 { return; }
        let input_pos = match (self.memory[self.pc] / 100) % 10 {
            0 => self.memory[self.pc + 1] as usize,
            2 => (self.rb + self.memory[self.pc + 1] as i32) as usize,
            _ => unreachable!(),
        };
        while self.memory.len() <= input_pos {
            self.memory.push(0);
        }
        self.memory[input_pos] = input;
        self.pc += 2;
        self.run()
    }

    pub fn run(&mut self) {
        if self.memory[self.pc] == 99 { return; }
        while let Some(mut op) = Operation::new(&mut self.memory, self.pc, self.rb) {
            match op.opcode {
                1 => self.memory[op.w] = op.v1 + op.v2,
                2 => self.memory[op.w] = op.v1 * op.v2,
                3 => break,
                4 => self.output.push(op.v1),
                5 => op.pc = if op.v1 != 0 { op.v2 as usize } else { op.pc + 3 },
                6 => op.pc = if op.v1 == 0 { op.v2 as usize } else { op.pc + 3 },
                7 => self.memory[op.w] = if op.v1 < op.v2 { 1 } else { 0 },
                8 => self.memory[op.w] = if op.v1 == op.v2 { 1 } else { 0 },
                9 => self.rb = op.rb,
                _ => unreachable!()
            }
            self.pc = op.pc;
        }
    }

    pub fn get_output(&mut self) -> Option<i64> {
        self.output.pop()
    }
}

#[derive(Debug)]
struct Operation {
    opcode: i64,
    v1: i64,
    v2: i64,
    w: usize,
    pc: usize,
    rb: i32,
}

impl Operation {
    fn new(mem: &mut Vec<i64>, pc: usize, rb: i32) -> Option<Self> {
        while mem.len() <= pc + 3 {
            mem.push(0);
        }
        let opcode = mem[pc] % 100;
        if opcode == 99 { return None; }
        let w_mode = (mem[pc] / 10000) % 10;
        if opcode == 3 {
            let w = match w_mode {
                0 => mem[pc + 3] as usize,
                2 => (rb + mem[pc + 1] as i32) as usize,
                _ => unreachable!(),
            };
            while mem.len() <= w {
                mem.push(0);
            }
            return Some(Operation { opcode, v1: 0, v2: 0, w, pc: pc + 2, rb });
        }
        let v1 = match (mem[pc] / 100) % 10 {
            0 => {
                while mem.len() as i64 <= mem[pc + 1] {
                    mem.push(0);
                }
                mem[mem[pc + 1] as usize]
            },
            1 => mem[pc + 1],
            2 => {
                while mem.len() as i32 <= rb + mem[pc + 1] as i32 {
                    mem.push(0);
                }
                mem[(rb + mem[pc + 1] as i32) as usize]
            },
            _ => unreachable!(),
        };
        if opcode == 4 {
            return Some(Operation { opcode, v1, v2: 0, w: 0, pc: pc + 2, rb });
        }
        if opcode == 9 {
            return Some(Operation { opcode, v1: 0, v2: 0, w: 0, pc: pc + 2, rb: rb + v1 as i32 });
        }
        let v2 = match (mem[pc] / 1000) % 10 {
            0 => {
                while mem.len() as i64 <= mem[pc + 2] {
                    mem.push(0);
                }
                mem[mem[pc + 2] as usize]
            },
            1 => mem[pc + 2],
            2 => {
                while mem.len() as i32 <= rb + mem[pc + 2] as i32 {
                    mem.push(0);
                }
                mem[(rb + mem[pc + 2] as i32) as usize]
            },
            _ => unreachable!(),
        };
        match opcode {
            1 | 2 | 7 | 8 => {
                let w = match w_mode {
                    0 => mem[pc + 3] as usize,
                    2 => (rb + mem[pc + 3] as i32) as usize,
                    _ => unreachable!(),
                };
                while mem.len() <= w {
                    mem.push(0);
                }
                Some(Operation { opcode, v1, v2, w, pc: pc + 4, rb })
            },
            5 | 6 => Some(Operation { opcode, v1, v2, w: 0, pc, rb }),
            _ => panic!("Error: opcode = {}", opcode)
        }
    }
}
